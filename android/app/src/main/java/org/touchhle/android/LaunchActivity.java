/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.content.Intent;
import android.content.SharedPreferences;
import android.net.Uri;
import android.os.Bundle;

import androidx.appcompat.app.AppCompatActivity;
import androidx.documentfile.provider.DocumentFile;

/**
 * Main launcher activity that determines where to redirect the user
 */
public class LaunchActivity extends AppCompatActivity {

    private static final String PREF_FOLDER_URI = "folder_uri";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Prune old/stale cached game copies to avoid storage bloat
        GameFileResolver.pruneCache(this, /*maxAgeMs=*/3L*24*60*60*1000, /*maxTotalBytes=*/1L*1024*1024*1024);

        // Check if we have a previously selected folder
        SharedPreferences prefs = getSharedPreferences("touchhle_prefs", MODE_PRIVATE);
        String folderUriString = prefs.getString(PREF_FOLDER_URI, null);

        Intent targetIntent;

        if (folderUriString != null) {
            // Check if the folder still exists and we have permission
            try {
                Uri folderUri = Uri.parse(folderUriString);
                DocumentFile folder = DocumentFile.fromTreeUri(this, folderUri);
                
                if (folder != null && folder.exists()) {
                    // Folder exists, go directly to game list
                    targetIntent = new Intent(this, GameListActivity.class);
                    targetIntent.putExtra("folder_uri", folderUriString);
                } else {
                    // Folder doesn't exist anymore, go to folder selector
                    targetIntent = new Intent(this, FolderSelectorActivity.class);
                }
            } catch (Exception e) {
                // Invalid URI, go to folder selector
                targetIntent = new Intent(this, FolderSelectorActivity.class);
            }
        } else {
            // No folder selected yet, go to folder selector
            targetIntent = new Intent(this, FolderSelectorActivity.class);
        }

        startActivity(targetIntent);
        finish();
    }
}