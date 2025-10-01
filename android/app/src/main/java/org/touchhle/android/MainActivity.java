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
import android.provider.DocumentsContract;
import android.util.Log;

import androidx.documentfile.provider.DocumentFile;

import org.libsdl.app.SDLActivity;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;

/**
 * A wrapper class over SDLActivity that runs the touchHLE emulator
 */
public class MainActivity extends SDLActivity {
    
    private static final String TAG = "MainActivity";
    private static final String CACHE_DIR = "runtime_games";

    private File tempGameFile;
    private String tempGamePath;
    private String selectedGameName;

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
                    String gamePath = resolveGamePath(Uri.parse(gameUriString));

                    if (gamePath != null) {
                        tempGamePath = gamePath;
                        if (gameName == null || gameName.trim().isEmpty()) {
                            selectedGameName = new File(gamePath).getName();
                        } else {
                            selectedGameName = gameName;
                        }
                        TouchHLENative.prepareLaunch(tempGamePath, selectedGameName);
                        Log.d(TAG, "Prepared game path: " + gamePath);
                    } else {
                        Log.e(TAG, "Failed to resolve game path");
                        TouchHLENative.clearLaunch();
                        finish();
                        return;
                    }
                } catch (Exception e) {
                    Log.e(TAG, "Error setting up game: " + e.getMessage(), e);
                    TouchHLENative.clearLaunch();
                    finish();
                    return;
                }
            } else {
                Log.e(TAG, "No game specified, finishing");
                TouchHLENative.clearLaunch();
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

    @Override
    protected String[] getArguments() {
        if (tempGamePath != null) {
            return new String[]{tempGamePath};
        }
        return new String[0];
    }
    
    private String resolveGamePath(Uri gameUri) {
        if (gameUri == null) {
            return null;
        }

        if ("file".equalsIgnoreCase(gameUri.getScheme())) {
            File file = new File(gameUri.getPath());
            if (file.exists()) {
                tempGameFile = null;
                return file.getAbsolutePath();
            }
            Log.e(TAG, "File URI does not exist: " + gameUri);
            return null;
        }

        if ("content".equalsIgnoreCase(gameUri.getScheme())) {
            DocumentFile gameFile = DocumentFile.fromSingleUri(this, gameUri);
            if (gameFile == null || !gameFile.exists()) {
                Log.e(TAG, "Game file does not exist or is not accessible");
                return null;
            }

            String absolutePath = resolveDocumentFilePath(gameFile);
            if (absolutePath != null) {
                File candidate = new File(absolutePath);
                if (candidate.exists() && candidate.canRead()) {
                    tempGameFile = null;
                    return candidate.getAbsolutePath();
                }
                Log.w(TAG, "Resolved path inaccessible, falling back to cached copy: " + absolutePath);
            }

            String cached = copyDocumentToCache(gameFile);
            if (cached != null) {
                return cached;
            }
            Log.e(TAG, "Content URI could not be cached: " + gameUri);
            return null;
        }

        Log.e(TAG, "Unsupported URI scheme: " + gameUri.getScheme());
        return null;
    }

    private String resolveDocumentFilePath(DocumentFile documentFile) {
        Uri uri = documentFile.getUri();
        if (!DocumentsContract.isDocumentUri(this, uri)) {
            return null;
        }

        try {
            String documentId = DocumentsContract.getDocumentId(uri);
            if (documentId == null) {
                return null;
            }

            File baseDir = getExternalFilesDir(null);
            if (baseDir == null) {
                return null;
            }

            String root = org.touchhle.android.DocumentsProvider.ROOT_ID;
            if (documentId.equals(root)) {
                return baseDir.getAbsolutePath();
            }

            String prefix = root + "/";
            if (documentId.startsWith(prefix)) {
                String relative = documentId.substring(prefix.length());
                File target = new File(baseDir, relative);
                if (target.exists()) {
                    return target.getAbsolutePath();
                }
            }
        } catch (IllegalArgumentException e) {
            Log.e(TAG, "Failed to parse document URI: " + uri, e);
        }
        return null;
    }

    private String copyDocumentToCache(DocumentFile documentFile) {
        try (InputStream inputStream = getContentResolver().openInputStream(documentFile.getUri())) {
            if (inputStream == null) {
                return null;
            }
            File cacheDir = new File(getCacheDir(), CACHE_DIR);
            if (!cacheDir.exists() && !cacheDir.mkdirs()) {
                Log.e(TAG, "Failed to create cache directory: " + cacheDir.getAbsolutePath());
                return null;
            }

            String fileName = documentFile.getName();
            if (fileName == null || fileName.trim().isEmpty()) {
                fileName = "touchhle_game";
            }
            File tempFile = File.createTempFile("game_", "_" + fileName, cacheDir);

            try (FileOutputStream outputStream = new FileOutputStream(tempFile)) {
                byte[] buffer = new byte[8192];
                int bytesRead;
                while ((bytesRead = inputStream.read(buffer)) != -1) {
                    outputStream.write(buffer, 0, bytesRead);
                }
                outputStream.flush();
            }

            if (tempGameFile != null && tempGameFile.exists()) {
                // delete previous cache to avoid leaks
                // ignoring result intentionally
                tempGameFile.delete();
            }
            tempGameFile = tempFile;
            return tempFile.getAbsolutePath();
        } catch (IOException e) {
            Log.e(TAG, "Failed to cache document: " + documentFile.getName(), e);
        }
        return null;
    }

    @Override
    protected void onDestroy() {
        cleanUpTempGame();
        super.onDestroy();
    }

    private void cleanUpTempGame() {
        if (tempGameFile != null && tempGameFile.exists()) {
            if (!tempGameFile.delete()) {
                Log.w(TAG, "Failed to delete cached game: " + tempGameFile.getAbsolutePath());
            }
        }
        tempGameFile = null;
        tempGamePath = null;
        selectedGameName = null;
        TouchHLENative.clearLaunch();
    }
}
