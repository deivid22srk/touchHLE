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
import android.view.View;
import android.view.WindowManager;

import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.core.view.WindowInsetsControllerCompat;

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
    private boolean forceFullscreen;

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
                        forceFullscreen = SettingsManager.getFullscreen(this);
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
        applyFullscreenMode();
    }

    @Override
    protected void onResume() {
        super.onResume();
        forceFullscreen = SettingsManager.getFullscreen(this);
        applyFullscreenMode();
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            applyFullscreenMode();
        }
    }

    private void applyFullscreenMode() {
        View decorView = getWindow().getDecorView();
        WindowInsetsControllerCompat controller = WindowCompat.getInsetsController(getWindow(), decorView);
        if (forceFullscreen) {
            WindowCompat.setDecorFitsSystemWindows(getWindow(), false);
            decorView.setSystemUiVisibility(
                    View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                            | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                            | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                            | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                            | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                            | View.SYSTEM_UI_FLAG_FULLSCREEN
            );
            if (controller != null) {
                controller.setSystemBarsBehavior(WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                controller.hide(WindowInsetsCompat.Type.systemBars());
            }
            getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        } else {
            WindowCompat.setDecorFitsSystemWindows(getWindow(), true);
            decorView.setSystemUiVisibility(View.SYSTEM_UI_FLAG_VISIBLE);
            if (controller != null) {
                controller.show(WindowInsetsCompat.Type.systemBars());
            }
            getWindow().clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        }
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
