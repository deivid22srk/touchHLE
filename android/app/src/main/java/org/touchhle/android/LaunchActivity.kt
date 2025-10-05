/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
import androidx.documentfile.provider.DocumentFile

class LaunchActivity : AppCompatActivity() {

    companion object {
        private const val PREF_FOLDER_URI = "folder_uri"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        // Install Material 3 splash screen
        installSplashScreen()
        
        super.onCreate(savedInstanceState)

        // Prune old/stale cached game copies to avoid storage bloat
        GameFileResolver.pruneCache(
            this,
            maxAgeMs = 3L * 24 * 60 * 60 * 1000, // 3 days
            maxTotalBytes = 1L * 1024 * 1024 * 1024 // 1 GB
        )

        // Check if we have a previously selected folder
        val prefs = getSharedPreferences("touchhle_prefs", MODE_PRIVATE)
        val folderUriString = prefs.getString(PREF_FOLDER_URI, null)

        val targetIntent = if (folderUriString != null) {
            // Check if the folder still exists and we have permission
            try {
                val folderUri = Uri.parse(folderUriString)
                val folder = DocumentFile.fromTreeUri(this, folderUri)

                if (folder != null && folder.exists()) {
                    // Folder exists, go directly to game list
                    Intent(this, GameListActivity::class.java).apply {
                        putExtra("folder_uri", folderUriString)
                    }
                } else {
                    // Folder doesn't exist anymore, go to folder selector
                    Intent(this, FolderSelectorActivity::class.java)
                }
            } catch (e: Exception) {
                // Invalid URI, go to folder selector
                Intent(this, FolderSelectorActivity::class.java)
            }
        } else {
            // No folder selected yet, go to folder selector
            Intent(this, FolderSelectorActivity::class.java)
        }

        startActivity(targetIntent)
        finish()
    }
}
