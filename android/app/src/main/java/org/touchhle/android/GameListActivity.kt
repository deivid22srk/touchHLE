/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android

import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.os.ParcelFileDescriptor
import android.text.Editable
import android.text.TextWatcher
import android.util.Base64
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import android.view.View
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.WindowCompat
import androidx.documentfile.provider.DocumentFile
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.appbar.MaterialToolbar
import com.google.android.material.button.MaterialButton
import com.google.android.material.textfield.TextInputEditText
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONException
import org.json.JSONObject
import java.io.File
import java.io.IOException
import java.util.Locale

class GameListActivity : AppCompatActivity(), GameAdapter.OnGameClickListener {

    companion object {
        private const val TAG = "GameListActivity"
    }

    private lateinit var gamesRecyclerView: RecyclerView
    private lateinit var gamesCountText: TextView
    private lateinit var searchEditText: TextInputEditText
    private lateinit var emptyStateLayout: View
    private lateinit var changeFolderButton: MaterialButton

    private lateinit var gameAdapter: GameAdapter
    private val allGames = mutableListOf<GameInfo>()
    private val filteredGames = mutableListOf<GameInfo>()

    private var folderUri: Uri? = null
    private lateinit var defaultGameIcon: Bitmap

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Enable edge-to-edge layout
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            WindowCompat.setDecorFitsSystemWindows(window, false)
        }
        
        setContentView(R.layout.activity_game_list)

        initializeViews()
        setupRecyclerView()
        setupSearch()
        setupClickListeners()

        val folderUriString = intent.getStringExtra("folder_uri")
        if (folderUriString != null) {
            folderUri = Uri.parse(folderUriString)
            scanForGames()
        } else {
            startActivity(Intent(this, FolderSelectorActivity::class.java))
            finish()
        }
    }

    private fun initializeViews() {
        val toolbar = findViewById<MaterialToolbar>(R.id.toolbar)
        setSupportActionBar(toolbar)

        gamesRecyclerView = findViewById(R.id.gamesRecyclerView)
        gamesCountText = findViewById(R.id.gamesCountText)
        searchEditText = findViewById(R.id.searchEditText)
        emptyStateLayout = findViewById(R.id.emptyStateLayout)
        changeFolderButton = findViewById(R.id.changeFolderButton)
        defaultGameIcon = BitmapFactory.decodeResource(resources, R.drawable.icon)

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        toolbar.setNavigationOnClickListener {
            onBackPressedDispatcher.onBackPressed()
        }
    }

    private fun setupRecyclerView() {
        gameAdapter = GameAdapter(filteredGames, this)
        gamesRecyclerView.layoutManager = LinearLayoutManager(this)
        gamesRecyclerView.adapter = gameAdapter
    }

    private fun setupSearch() {
        searchEditText.addTextChangedListener(object : TextWatcher {
            override fun beforeTextChanged(s: CharSequence?, start: Int, count: Int, after: Int) {}

            override fun onTextChanged(s: CharSequence?, start: Int, before: Int, count: Int) {
                filterGames(s.toString())
            }

            override fun afterTextChanged(s: Editable?) {}
        })
    }

    private fun setupClickListeners() {
        changeFolderButton.setOnClickListener {
            startActivity(Intent(this, FolderSelectorActivity::class.java))
            finish()
        }
    }

    private fun scanForGames() {
        gamesCountText.setText(R.string.scanning_games)

        CoroutineScope(Dispatchers.IO).launch {
            val games = findGamesInFolder(folderUri)

            withContext(Dispatchers.Main) {
                allGames.clear()
                allGames.addAll(games)
                filterGames(searchEditText.text.toString())
                updateUI()
            }
        }
    }

    private fun findGamesInFolder(folderUri: Uri?): List<GameInfo> {
        val games = mutableListOf<GameInfo>()

        folderUri?.let { uri ->
            val folder = DocumentFile.fromTreeUri(this, uri)
            if (folder != null && folder.exists() && folder.isDirectory) {
                scanDirectory(folder, games)
            }
        }

        return games
    }

    private fun scanDirectory(directory: DocumentFile, games: MutableList<GameInfo>) {
        directory.listFiles().forEach { file ->
            if (file.isDirectory) {
                scanDirectory(file, games)
            } else {
                file.name?.let { fileName ->
                    val lowerName = fileName.lowercase(Locale.ROOT)
                    if (lowerName.endsWith(".ipa") || lowerName.endsWith(".app")) {
                        createGameInfoBasic(file)?.let { info ->
                            games.add(info)
                            loadMetadataAsync(file, info)
                        }
                    }
                }
            }
        }
    }

    private fun createGameInfoBasic(file: DocumentFile): GameInfo? {
        val fileName = file.name ?: return null
        val baseName = fileName.substringBeforeLast('.')
        val type = if (fileName.lowercase(Locale.ROOT).endsWith(".ipa")) {
            GameInfo.Type.IPA
        } else {
            GameInfo.Type.APP
        }
        val sizeBytes = file.length()
        val sizeString = formatFileSize(sizeBytes)

        return GameInfo(baseName, "Unknown Version", sizeString, file.uri, type).apply {
            icon = defaultGameIcon
        }
    }

    private fun loadMetadataAsync(file: DocumentFile, info: GameInfo) {
        CoroutineScope(Dispatchers.IO).launch {
            val metadata = fetchMetadataFast(file)
            metadata?.let { meta ->
                withContext(Dispatchers.Main) {
                    meta.displayName?.takeIf { it.isNotEmpty() }?.let {
                        info.name = it
                    }
                    meta.version?.takeIf { it.isNotEmpty() }?.let {
                        info.version = it
                    }
                    meta.iconBitmap?.let {
                        info.icon = it
                    }
                    info.bundleIdentifier = meta.bundleIdentifier
                    gameAdapter.notifyDataSetChanged()
                    updateUI()
                }
            }
        }
    }

    private fun fetchMetadataFast(file: DocumentFile): NativeMetadata? {
        return try {
            val name = file.name
            if (name != null && !file.isDirectory && name.lowercase(Locale.ROOT).endsWith(".ipa")) {
                contentResolver.openFileDescriptor(file.uri, "r")?.use { pfd ->
                    val fdPath = "/proc/self/fd/${pfd.fd}"
                    val json = TouchHLENative.inspectBundle(fdPath) ?: return null
                    parseMetadata(json, file)
                }
            } else {
                fetchMetadata(file)
            }
        } catch (e: Exception) {
            Log.w(TAG, "Metadata fast path failed for ${file.name}", e)
            null
        }
    }

    private fun fetchMetadata(file: DocumentFile): NativeMetadata? {
        var tempCopy: File? = null
        return try {
            tempCopy = GameFileResolver.copyForInspection(this, file) ?: return null
            val json = TouchHLENative.inspectBundle(tempCopy.absolutePath) ?: return null
            parseMetadata(json, file)
        } catch (e: IOException) {
            Log.w(TAG, "Unable to inspect bundle ${file.name}", e)
            null
        } catch (e: JSONException) {
            Log.w(TAG, "Unable to inspect bundle ${file.name}", e)
            null
        } finally {
            tempCopy?.let { GameFileResolver.deleteRecursively(it) }
        }
    }

    private fun parseMetadata(json: String, file: DocumentFile): NativeMetadata? {
        return try {
            val jsonObject = JSONObject(json)
            NativeMetadata().apply {
                displayName = jsonObject.optString("display_name", null)
                version = jsonObject.optString("version", null)
                bundleIdentifier = jsonObject.optString("bundle_identifier", null)

                val iconBase64 = jsonObject.optString("icon_png", null)
                if (!iconBase64.isNullOrEmpty()) {
                    try {
                        val iconBytes = Base64.decode(iconBase64, Base64.DEFAULT)
                        iconBitmap = BitmapFactory.decodeByteArray(iconBytes, 0, iconBytes.size)
                    } catch (decodeError: IllegalArgumentException) {
                        Log.w(TAG, "Failed to decode icon for ${file.name}", decodeError)
                    }
                }
            }
        } catch (e: JSONException) {
            null
        }
    }

    private fun formatFileSize(bytes: Long): String {
        if (bytes < 1024) return "$bytes B"
        val exp = (Math.log(bytes.toDouble()) / Math.log(1024.0)).toInt()
        val pre = "KMGTPE"[exp - 1]
        return String.format(Locale.getDefault(), "%.1f %sB", bytes / Math.pow(1024.0, exp.toDouble()), pre)
    }

    private fun filterGames(query: String) {
        filteredGames.clear()

        if (query.trim().isEmpty()) {
            filteredGames.addAll(allGames)
        } else {
            val lowerQuery = query.lowercase(Locale.ROOT)
            allGames.forEach { game ->
                if (game.name.lowercase(Locale.ROOT).contains(lowerQuery)) {
                    filteredGames.add(game)
                }
            }
        }

        gameAdapter.notifyDataSetChanged()
        updateUI()
    }

    private fun updateUI() {
        val gameCount = filteredGames.size

        if (gameCount == 0) {
            gamesRecyclerView.visibility = View.GONE
            emptyStateLayout.visibility = View.VISIBLE

            if (allGames.isEmpty()) {
                gamesCountText.setText(R.string.no_games_found)
            } else {
                gamesCountText.text = "Nenhum jogo corresponde à pesquisa"
            }
        } else {
            gamesRecyclerView.visibility = View.VISIBLE
            emptyStateLayout.visibility = View.GONE
            gamesCountText.text = getString(R.string.games_found, gameCount)
        }
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        menuInflater.inflate(R.menu.game_list_menu, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        return when (item.itemId) {
            android.R.id.home -> {
                onBackPressed()
                true
            }
            R.id.action_refresh -> {
                scanForGames()
                true
            }
            R.id.action_settings -> {
                startActivity(Intent(this, SettingsActivity::class.java))
                true
            }
            R.id.action_about -> {
                true
            }
            else -> super.onOptionsItemSelected(item)
        }
    }

    override fun onGameClick(gameInfo: GameInfo) {
        launchGame(gameInfo)
    }

    override fun onGameMenuClick(gameInfo: GameInfo) {
        // Placeholder for future context actions
    }

    private fun launchGame(gameInfo: GameInfo) {
        try {
            val intent = Intent(this, MainActivity::class.java).apply {
                putExtra("game_uri", gameInfo.fileUri.toString())
                putExtra("game_name", gameInfo.name)
            }
            startActivity(intent)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to launch game", e)
        }
    }

    private data class NativeMetadata(
        var displayName: String? = null,
        var version: String? = null,
        var bundleIdentifier: String? = null,
        var iconBitmap: Bitmap? = null
    )
}
