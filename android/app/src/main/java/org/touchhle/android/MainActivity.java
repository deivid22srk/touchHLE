/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Parts of this file are derived from SDL 2's Android project template, which
 * has a different license. Please see vendor/SDL/LICENSE.txt for details.
 */
package org.touchhle.android;

import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.util.Log;

import androidx.documentfile.provider.DocumentFile;

import org.libsdl.app.SDLActivity;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;

/**
 * A wrapper class over SDLActivity that runs the touchHLE emulator
 */
public class MainActivity extends SDLActivity {
    
    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        // Get game information from intent
        Intent intent = getIntent();
        if (intent != null) {
            String gameUriString = intent.getStringExtra("game_uri");
            String gameName = intent.getStringExtra("game_name");
            
            if (gameUriString != null && gameName != null) {
                Log.d(TAG, "Received game URI: " + gameUriString);
                Log.d(TAG, "Received game name: " + gameName);
                
                try {
                    // Copy the game file to internal storage so touchHLE can access it
                    String gamePath = copyGameToInternalStorage(Uri.parse(gameUriString), gameName);
                    
                    if (gamePath != null) {
                        // Set the game path in the native code
                        TouchHLENative.setGamePath(gamePath, gameName);
                        Log.d(TAG, "Set game path: " + gamePath);
                    } else {
                        Log.e(TAG, "Failed to copy game to internal storage");
                        finish();
                        return;
                    }
                } catch (Exception e) {
                    Log.e(TAG, "Error setting up game: " + e.getMessage(), e);
                    finish();
                    return;
                }
            } else {
                Log.e(TAG, "No game specified, finishing");
                finish();
                return;
            }
        }
        
        super.onCreate(savedInstanceState);
    }

    @Override
    protected String[] getLibraries() {
        return new String[]{
            "SDL2",
            "touchHLE"
        };
    }
    
    private String copyGameToInternalStorage(Uri gameUri, String gameName) {
        try {
            DocumentFile gameFile = DocumentFile.fromSingleUri(this, gameUri);
            if (gameFile == null || !gameFile.exists()) {
                Log.e(TAG, "Game file does not exist or is not accessible");
                return null;
            }
            
            // Create a temporary file in internal storage
            File internalDir = new File(getFilesDir(), "temp_games");
            internalDir.mkdirs();
            
            String fileName = gameFile.getName();
            if (fileName == null) {
                fileName = gameName + ".app"; // fallback
            }
            
            File tempFile = new File(internalDir, fileName);
            
            // Copy the content
            try (InputStream inputStream = getContentResolver().openInputStream(gameUri);
                 FileOutputStream outputStream = new FileOutputStream(tempFile)) {
                
                if (inputStream == null) {
                    Log.e(TAG, "Could not open input stream for game file");
                    return null;
                }
                
                byte[] buffer = new byte[8192];
                int bytesRead;
                while ((bytesRead = inputStream.read(buffer)) != -1) {
                    outputStream.write(buffer, 0, bytesRead);
                }
            }
            
            Log.d(TAG, "Copied game file to: " + tempFile.getAbsolutePath());
            return tempFile.getAbsolutePath();
            
        } catch (Exception e) {
            Log.e(TAG, "Error copying game to internal storage: " + e.getMessage(), e);
            return null;
        }
    }
    
    @Override
    protected void onDestroy() {
        // Clean up the temporary game file
        try {
            if (TouchHLENative.hasGamePath()) {
                String gamePath = TouchHLENative.getGamePath();
                if (gamePath != null) {
                    File gameFile = new File(gamePath);
                    if (gameFile.exists() && gameFile.delete()) {
                        Log.d(TAG, "Cleaned up temporary game file: " + gamePath);
                    }
                }
                TouchHLENative.clearGamePath();
            }
        } catch (Exception e) {
            Log.e(TAG, "Error cleaning up: " + e.getMessage());
        }
        
        super.onDestroy();
    }
}
