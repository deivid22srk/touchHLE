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

import org.libsdl.app.SDLActivity;

import java.io.File;
import java.io.IOException;

/**
 * A wrapper class over SDLActivity that runs the touchHLE emulator
 */
public class MainActivity extends SDLActivity {

    private static final String TAG = "MainActivity";

    private File tempGameFile;
    private String tempGamePath;
    private String selectedGameName;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        Intent intent = getIntent();
        if (intent != null) {
            String gameUriString = intent.getStringExtra("game_uri");
            String gameName = intent.getStringExtra("game_name");

            if (gameUriString != null) {
                Log.d(TAG, "Received game URI: " + gameUriString);
                if (gameName != null) {
                    Log.d(TAG, "Received game name: " + gameName);
                }

                try {
                    GameFileResolver.Result resolution = GameFileResolver.resolveForLaunch(
                            this,
                            Uri.parse(gameUriString)
                    );

                    if (resolution != null) {
                        tempGamePath = resolution.getPath();
                        tempGameFile = resolution.getCacheFile();
                        selectedGameName = (gameName != null && !gameName.trim().isEmpty())
                                ? gameName
                                : new File(tempGamePath).getName();
                        String[] optionArgs = SettingsManager.buildOptionArgs(this);
                        TouchHLENative.prepareLaunch(tempGamePath, selectedGameName, optionArgs);
                        Log.d(TAG, "Prepared game path: " + tempGamePath);
                    } else {
                        Log.e(TAG, "Unable to resolve path for URI: " + gameUriString);
                        TouchHLENative.clearLaunch();
                        finish();
                        return;
                    }
                } catch (IOException e) {
                    Log.e(TAG, "Error resolving game URI", e);
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

    @Override
    protected void onDestroy() {
        cleanUpTempGame();
        super.onDestroy();
    }

    private void cleanUpTempGame() {
        if (tempGameFile != null) {
            GameFileResolver.deleteRecursively(tempGameFile);
        }
        tempGameFile = null;
        tempGamePath = null;
        selectedGameName = null;
        TouchHLENative.clearLaunch();
    }
}
