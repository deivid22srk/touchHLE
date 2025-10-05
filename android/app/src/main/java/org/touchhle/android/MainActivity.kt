/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Parts of this file are derived from SDL 2's Android project template, which
 * has a different license. Please see vendor/SDL/LICENSE.txt for details.
 */
package org.touchhle.android

import android.content.SharedPreferences
import android.net.Uri
import android.os.Bundle
import android.os.ParcelFileDescriptor
import android.util.Log
import android.view.Gravity
import android.view.View
import android.view.ViewGroup
import android.view.WindowManager
import android.widget.FrameLayout
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.drawerlayout.widget.DrawerLayout
import com.google.android.material.button.MaterialButton
import com.google.android.material.switchmaterial.SwitchMaterial
import org.libsdl.app.SDLActivity
import java.io.File
import java.io.IOException

class MainActivity : SDLActivity() {

    companion object {
        private const val TAG = "MainActivity"
        private const val PREFS_NAME = "PerformancePrefs"
        private const val PREF_SHOW_FPS = "show_fps"
        private const val PREF_SHOW_RAM = "show_ram"
    }

    private var tempGameFile: File? = null
    private var tempPfd: ParcelFileDescriptor? = null
    private var tempGamePath: String? = null
    private var selectedGameName: String? = null
    private var forceFullscreen: Boolean = false

    private var drawerLayout: DrawerLayout? = null
    private var performanceOverlay: PerformanceOverlayView? = null
    private var fpsSwitch: SwitchMaterial? = null
    private var ramSwitch: SwitchMaterial? = null
    private var exitGameButton: MaterialButton? = null
    private lateinit var preferences: SharedPreferences

    override fun onCreate(savedInstanceState: Bundle?) {
        preferences = getSharedPreferences(PREFS_NAME, MODE_PRIVATE)
        intent?.let { intent ->
            val gameUriString = intent.getStringExtra("game_uri")
            val gameName = intent.getStringExtra("game_name")

            if (gameUriString != null) {
                Log.d(TAG, "Received game URI: $gameUriString")
                gameName?.let { Log.d(TAG, "Received game name: $it") }

                try {
                    val resolution = GameFileResolver.resolveForLaunch(
                        this,
                        Uri.parse(gameUriString)
                    )

                    if (resolution != null) {
                        tempGamePath = resolution.path
                        tempGameFile = resolution.cacheFile
                        tempPfd = resolution.pfd
                        selectedGameName = if (!gameName.isNullOrBlank()) {
                            gameName
                        } else {
                            File(resolution.path).name
                        }
                        forceFullscreen = SettingsManager.getFullscreen(this)
                        val optionArgs = SettingsManager.buildOptionArgs(this)
                        TouchHLENative.prepareLaunch(resolution.path, selectedGameName!!, optionArgs)
                        Log.d(TAG, "Prepared game path: ${resolution.path}")
                    } else {
                        Log.e(TAG, "Unable to resolve path for URI: $gameUriString")
                        TouchHLENative.clearLaunch()
                        finish()
                        return
                    }
                } catch (e: IOException) {
                    Log.e(TAG, "Error resolving game URI", e)
                    TouchHLENative.clearLaunch()
                    finish()
                    return
                }
            } else {
                Log.e(TAG, "No game specified, finishing")
                TouchHLENative.clearLaunch()
                finish()
                return
            }
        }

        super.onCreate(savedInstanceState)
        setupDrawerAndOverlay()
        applyFullscreenMode()
    }

    override fun onResume() {
        super.onResume()
        forceFullscreen = SettingsManager.getFullscreen(this)
        applyFullscreenMode()
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) {
            applyFullscreenMode()
        }
    }

    private fun applyFullscreenMode() {
        val decorView = window.decorView
        val controller = WindowCompat.getInsetsController(window, decorView)

        if (forceFullscreen) {
            WindowCompat.setDecorFitsSystemWindows(window, false)
            @Suppress("DEPRECATION")
            decorView.systemUiVisibility = (
                View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                or View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                or View.SYSTEM_UI_FLAG_FULLSCREEN
            )
            controller?.apply {
                systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
                hide(WindowInsetsCompat.Type.systemBars())
            }
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        } else {
            WindowCompat.setDecorFitsSystemWindows(window, true)
            @Suppress("DEPRECATION")
            decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_VISIBLE
            controller?.show(WindowInsetsCompat.Type.systemBars())
            window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        }
    }

    override fun getLibraries(): Array<String> {
        return arrayOf("SDL2", "touchHLE")
    }

    override fun getArguments(): Array<String> {
        return tempGamePath?.let { arrayOf(it) } ?: emptyArray()
    }

    override fun onDestroy() {
        cleanUpTempGame()
        super.onDestroy()
    }

    private fun cleanUpTempGame() {
        tempGameFile?.let { GameFileResolver.deleteRecursively(it) }
        tempPfd?.let {
            try { it.close() } catch (ignored: Exception) {}
        }
        tempGameFile = null
        tempPfd = null
        tempGamePath = null
        selectedGameName = null
        TouchHLENative.clearLaunch()
    }

    override fun setContentView(view: View?) {
        super.setContentView(R.layout.activity_main)

        drawerLayout = findViewById(R.id.drawerLayout)
        val mainContent = findViewById<FrameLayout>(R.id.mainContent)

        view?.let { sdlView ->
            mainContent?.addView(
                sdlView, 0,
                FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                )
            )
        }
    }

    private fun setupDrawerAndOverlay() {
        post {
            if (drawerLayout == null) {
                drawerLayout = findViewById(R.id.drawerLayout)
            }

            val mainContent = findViewById<FrameLayout>(R.id.mainContent)
            if (mainContent != null && performanceOverlay == null) {
                performanceOverlay = PerformanceOverlayView(this).apply {
                    val params = FrameLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT,
                        ViewGroup.LayoutParams.MATCH_PARENT
                    )
                    mainContent.addView(this, params)
                }
            }

            findViewById<View>(R.id.navigationView)?.let { navigationView ->
                fpsSwitch = navigationView.findViewById(R.id.fpsSwitch)
                ramSwitch = navigationView.findViewById(R.id.ramSwitch)
                exitGameButton = navigationView.findViewById(R.id.exitGameButton)

                exitGameButton?.setOnClickListener {
                    finish()
                }

                fpsSwitch?.let { fps ->
                    ramSwitch?.let { ram ->
                        val showFps = preferences.getBoolean(PREF_SHOW_FPS, false)
                        val showRam = preferences.getBoolean(PREF_SHOW_RAM, false)

                        fps.isChecked = showFps
                        ram.isChecked = showRam

                        performanceOverlay?.apply {
                            this.showFps = showFps
                            this.showRam = showRam
                        }

                        fps.setOnCheckedChangeListener { _, isChecked ->
                            performanceOverlay?.showFps = isChecked
                            preferences.edit().putBoolean(PREF_SHOW_FPS, isChecked).apply()
                            drawerLayout?.closeDrawer(Gravity.START)
                        }

                        ram.setOnCheckedChangeListener { _, isChecked ->
                            performanceOverlay?.showRam = isChecked
                            preferences.edit().putBoolean(PREF_SHOW_RAM, isChecked).apply()
                            drawerLayout?.closeDrawer(Gravity.START)
                        }
                    }
                }
            }
        }
    }

    private fun post(runnable: () -> Unit) {
        window.decorView.post(runnable)
    }
}
