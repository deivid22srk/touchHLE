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
import android.content.SharedPreferences;
import android.net.Uri;
import android.os.Bundle;
import android.util.Log;
import android.os.ParcelFileDescriptor;
import android.view.Gravity;
import android.view.View;
import android.view.ViewGroup;
import android.view.WindowManager;
import android.widget.FrameLayout;

import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.core.view.WindowInsetsControllerCompat;
import androidx.drawerlayout.widget.DrawerLayout;

import com.google.android.material.button.MaterialButton;
import com.google.android.material.switchmaterial.SwitchMaterial;

import org.libsdl.app.SDLActivity;

import java.io.File;
import java.io.IOException;

/**
 * A wrapper class over SDLActivity that runs the touchHLE emulator
 */
public class MainActivity extends SDLActivity {

    private static final String TAG = "MainActivity";
    private static final String PREFS_NAME = "PerformancePrefs";
    private static final String PREF_SHOW_FPS = "show_fps";
    private static final String PREF_SHOW_RAM = "show_ram";

    private File tempGameFile;
    private ParcelFileDescriptor tempPfd;
    private String tempGamePath;
    private String selectedGameName;
    private boolean forceFullscreen;
    
    private DrawerLayout drawerLayout;
    private PerformanceOverlayView performanceOverlay;
    private SwitchMaterial fpsSwitch;
    private SwitchMaterial ramSwitch;
    private MaterialButton exitGameButton;
    private SharedPreferences preferences;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        preferences = getSharedPreferences(PREFS_NAME, MODE_PRIVATE);
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
                        tempPfd = resolution.getPfd();
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
        setupDrawerAndOverlay();
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
        if (tempPfd != null) {
            try { tempPfd.close(); } catch (Exception ignored) {}
        }
        tempGameFile = null;
        tempPfd = null;
        tempGamePath = null;
        selectedGameName = null;
        TouchHLENative.clearLaunch();
    }
    
    @Override
    public void setContentView(View view) {
        super.setContentView(R.layout.activity_main);
        
        drawerLayout = findViewById(R.id.drawerLayout);
        FrameLayout mainContent = findViewById(R.id.mainContent);
        
        if (view != null && mainContent != null) {
            mainContent.addView(view, 0, new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            ));
        }
    }
    
    private void setupDrawerAndOverlay() {
        post(() -> {
            if (drawerLayout == null) {
                drawerLayout = findViewById(R.id.drawerLayout);
            }
            
            FrameLayout mainContent = findViewById(R.id.mainContent);
            if (mainContent != null && performanceOverlay == null) {
                performanceOverlay = new PerformanceOverlayView(this);
                FrameLayout.LayoutParams params = new FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                );
                mainContent.addView(performanceOverlay, params);
            }
            
            // Get views from NavigationView
            View navigationView = findViewById(R.id.navigationView);
            if (navigationView != null) {
                fpsSwitch = navigationView.findViewById(R.id.fpsSwitch);
                ramSwitch = navigationView.findViewById(R.id.ramSwitch);
                exitGameButton = navigationView.findViewById(R.id.exitGameButton);
                
                if (exitGameButton != null) {
                    exitGameButton.setOnClickListener(v -> {
                        finish();
                    });
                }
                
                if (fpsSwitch != null && ramSwitch != null) {
                    boolean showFps = preferences.getBoolean(PREF_SHOW_FPS, false);
                    boolean showRam = preferences.getBoolean(PREF_SHOW_RAM, false);
                    
                    fpsSwitch.setChecked(showFps);
                    ramSwitch.setChecked(showRam);
                    
                    if (performanceOverlay != null) {
                        performanceOverlay.setShowFps(showFps);
                        performanceOverlay.setShowRam(showRam);
                    }
                    
                    fpsSwitch.setOnCheckedChangeListener((buttonView, isChecked) -> {
                        if (performanceOverlay != null) {
                            performanceOverlay.setShowFps(isChecked);
                        }
                        preferences.edit().putBoolean(PREF_SHOW_FPS, isChecked).apply();
                        if (drawerLayout != null) {
                            drawerLayout.closeDrawer(Gravity.START);
                        }
                    });
                    
                    ramSwitch.setOnCheckedChangeListener((buttonView, isChecked) -> {
                        if (performanceOverlay != null) {
                            performanceOverlay.setShowRam(isChecked);
                        }
                        preferences.edit().putBoolean(PREF_SHOW_RAM, isChecked).apply();
                        if (drawerLayout != null) {
                            drawerLayout.closeDrawer(Gravity.START);
                        }
                    });
                }
            }
        });
    }
    
    private void post(Runnable runnable) {
        View decorView = getWindow().getDecorView();
        if (decorView != null) {
            decorView.post(runnable);
        }
    }
}
