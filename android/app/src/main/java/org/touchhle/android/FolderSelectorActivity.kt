/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.provider.DocumentsContract
import android.widget.TextView
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.documentfile.provider.DocumentFile
import com.google.android.material.button.MaterialButton

class FolderSelectorActivity : AppCompatActivity() {

    companion object {
        private const val PREF_FOLDER_URI = "folder_uri"
    }

    private lateinit var selectedFolderText: TextView
    private lateinit var continueButton: MaterialButton
    private lateinit var browseButton: MaterialButton
    private lateinit var fileManagerButton: MaterialButton

    private var selectedFolderUri: Uri? = null

    private val folderPickerLauncher = registerForActivityResult(
        ActivityResultContracts.StartActivityForResult()
    ) { result ->
        if (result.resultCode == Activity.RESULT_OK && result.data != null) {
            result.data?.data?.let { uri ->
                // Take persistable permission
                contentResolver.takePersistableUriPermission(
                    uri,
                    Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                )
                setSelectedFolder(uri)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_folder_selector)

        initializeViews()
        setupClickListeners()
        loadSavedFolder()
    }

    private fun initializeViews() {
        selectedFolderText = findViewById(R.id.selectedFolderText)
        continueButton = findViewById(R.id.continueButton)
        browseButton = findViewById(R.id.browseButton)
        fileManagerButton = findViewById(R.id.fileManagerButton)
    }

    private fun setupClickListeners() {
        browseButton.setOnClickListener {
            openFolderPicker()
        }

        fileManagerButton.setOnClickListener {
            openFileManager()
        }

        continueButton.setOnClickListener {
            selectedFolderUri?.let { uri ->
                val intent = Intent(this, GameListActivity::class.java).apply {
                    putExtra("folder_uri", uri.toString())
                }
                startActivity(intent)
                finish()
            }
        }
    }

    private fun openFolderPicker() {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
            addFlags(
                Intent.FLAG_GRANT_READ_URI_PERMISSION or
                Intent.FLAG_GRANT_WRITE_URI_PERMISSION or
                Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION or
                Intent.FLAG_GRANT_PREFIX_URI_PERMISSION
            )
        }
        folderPickerLauncher.launch(intent)
    }

    private fun openFileManager() {
        try {
            val intent = Intent(Intent.ACTION_VIEW).apply {
                data = DocumentsContract.buildRootsUri("$packageName.provider")
                addFlags(
                    Intent.FLAG_GRANT_READ_URI_PERMISSION or
                    Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                )
            }
            startActivity(intent)
        } catch (e: Exception) {
            try {
                val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE)
                folderPickerLauncher.launch(intent)
            } catch (ex: Exception) {
                // Could show a Snackbar here
            }
        }
    }

    private fun setSelectedFolder(folderUri: Uri) {
        selectedFolderUri = folderUri

        DocumentFile.fromTreeUri(this, folderUri)?.let { folder ->
            val folderName = folder.name?.takeIf { it.isNotEmpty() } ?: "Root Directory"
            selectedFolderText.text = getString(R.string.folder_selected, folderName)
            continueButton.isEnabled = true

            getSharedPreferences("touchhle_prefs", MODE_PRIVATE)
                .edit()
                .putString(PREF_FOLDER_URI, folderUri.toString())
                .apply()
        }
    }

    private fun loadSavedFolder() {
        val savedUriString = getSharedPreferences("touchhle_prefs", MODE_PRIVATE)
            .getString(PREF_FOLDER_URI, null)

        savedUriString?.let { uriString ->
            try {
                val savedUri = Uri.parse(uriString)
                DocumentFile.fromTreeUri(this, savedUri)?.let { folder ->
                    if (folder.exists()) {
                        setSelectedFolder(savedUri)
                    }
                }
            } catch (e: Exception) {
                // Invalid saved URI, ignore
            }
        }
    }
}
