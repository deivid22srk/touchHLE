/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.net.Uri;
import android.os.Bundle;
import android.text.Editable;
import android.text.TextWatcher;
import android.util.Base64;
import android.util.Log;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;
import androidx.documentfile.provider.DocumentFile;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;
import com.google.android.material.appbar.MaterialToolbar;
import com.google.android.material.button.MaterialButton;
import com.google.android.material.textfield.TextInputEditText;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class GameListActivity extends AppCompatActivity implements GameAdapter.OnGameClickListener {

    private static final String TAG = "GameListActivity";

    private RecyclerView gamesRecyclerView;
    private TextView gamesCountText;
    private TextInputEditText searchEditText;
    private View emptyStateLayout;
    private MaterialButton changeFolderButton;

    private GameAdapter gameAdapter;
    private final List<GameInfo> allGames = new ArrayList<>();
    private final List<GameInfo> filteredGames = new ArrayList<>();

    private Uri folderUri;
    private ExecutorService executorService;
    private Bitmap defaultGameIcon;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_game_list);

        initializeViews();
        setupRecyclerView();
        setupSearch();
        setupClickListeners();

        String folderUriString = getIntent().getStringExtra("folder_uri");
        if (folderUriString != null) {
            folderUri = Uri.parse(folderUriString);
            scanForGames();
        } else {
            startActivity(new Intent(this, FolderSelectorActivity.class));
            finish();
        }
    }

    private void initializeViews() {
        MaterialToolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);

        gamesRecyclerView = findViewById(R.id.gamesRecyclerView);
        gamesCountText = findViewById(R.id.gamesCountText);
        searchEditText = findViewById(R.id.searchEditText);
        emptyStateLayout = findViewById(R.id.emptyStateLayout);
        changeFolderButton = findViewById(R.id.changeFolderButton);
        defaultGameIcon = BitmapFactory.decodeResource(getResources(), R.drawable.icon);

        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
        }
        toolbar.setNavigationOnClickListener(v -> getOnBackPressedDispatcher().onBackPressed());

        executorService = Executors.newCachedThreadPool();
    }

    private void setupRecyclerView() {
        gameAdapter = new GameAdapter(filteredGames, this);
        gamesRecyclerView.setLayoutManager(new LinearLayoutManager(this));
        gamesRecyclerView.setAdapter(gameAdapter);
    }

    private void setupSearch() {
        searchEditText.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {
            }

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {
                filterGames(s.toString());
            }

            @Override
            public void afterTextChanged(Editable s) {
            }
        });
    }

    private void setupClickListeners() {
        changeFolderButton.setOnClickListener(v -> {
            startActivity(new Intent(this, FolderSelectorActivity.class));
            finish();
        });
    }

    private void scanForGames() {
        gamesCountText.setText(R.string.scanning_games);

        executorService.execute(() -> {
            List<GameInfo> games = findGamesInFolder(folderUri);

            runOnUiThread(() -> {
                allGames.clear();
                allGames.addAll(games);
                filterGames(searchEditText.getText().toString());
                updateUI();
            });
        });
    }

    private List<GameInfo> findGamesInFolder(Uri folderUri) {
        List<GameInfo> games = new ArrayList<>();

        DocumentFile folder = DocumentFile.fromTreeUri(this, folderUri);
        if (folder != null && folder.exists() && folder.isDirectory()) {
            scanDirectory(folder, games);
        }

        return games;
    }

    private void scanDirectory(DocumentFile directory, List<GameInfo> games) {
        DocumentFile[] files = directory.listFiles();
        if (files == null) {
            return;
        }
        for (DocumentFile file : files) {
            if (file.isDirectory()) {
                scanDirectory(file, games);
            } else {
                String fileName = file.getName();
                if (fileName == null) {
                    continue;
                }
                String lowerName = fileName.toLowerCase(Locale.ROOT);
                if (lowerName.endsWith(".ipa") || lowerName.endsWith(".app")) {
                    GameInfo info = createGameInfo(file);
                    if (info != null) {
                        games.add(info);
                    }
                }
            }
        }
    }

    private GameInfo createGameInfo(DocumentFile file) {
        String fileName = file.getName();
        if (fileName == null) {
            return null;
        }

        String baseName = fileName;
        int extensionIndex = baseName.lastIndexOf('.');
        if (extensionIndex > 0) {
            baseName = baseName.substring(0, extensionIndex);
        }

        String version = "Unknown Version";
        GameInfo.Type type = fileName.toLowerCase(Locale.ROOT).endsWith(".ipa")
                ? GameInfo.Type.IPA
                : GameInfo.Type.APP;

        long sizeBytes = file.length();
        String sizeString = formatFileSize(sizeBytes);

        NativeMetadata metadata = fetchMetadata(file);
        if (metadata != null) {
            if (metadata.displayName != null && !metadata.displayName.isEmpty()) {
                baseName = metadata.displayName;
            }
            if (metadata.version != null && !metadata.version.isEmpty()) {
                version = metadata.version;
            }
        }

        GameInfo info = new GameInfo(baseName, version, sizeString, file.getUri(), type);
        if (metadata != null && metadata.iconBitmap != null) {
            info.setIcon(metadata.iconBitmap);
        } else {
            info.setIcon(defaultGameIcon);
        }
        if (metadata != null) {
            info.setBundleIdentifier(metadata.bundleIdentifier);
        }
        return info;
    }

    private NativeMetadata fetchMetadata(DocumentFile file) {
        File tempCopy = null;
        try {
            tempCopy = GameFileResolver.copyForInspection(this, file);
            if (tempCopy == null) {
                return null;
            }
            String json = TouchHLENative.inspectBundle(tempCopy.getAbsolutePath());
            if (json == null || json.isEmpty()) {
                return null;
            }
            JSONObject object = new JSONObject(json);
            NativeMetadata metadata = new NativeMetadata();
            metadata.displayName = object.optString("display_name", null);
            metadata.version = object.optString("version", null);
            metadata.bundleIdentifier = object.optString("bundle_identifier", null);
            String iconBase64 = object.optString("icon_png", null);
            if (iconBase64 != null && !iconBase64.isEmpty()) {
                try {
                    byte[] iconBytes = Base64.decode(iconBase64, Base64.DEFAULT);
                    metadata.iconBitmap = BitmapFactory.decodeByteArray(iconBytes, 0, iconBytes.length);
                } catch (IllegalArgumentException decodeError) {
                    Log.w(TAG, "Failed to decode icon for " + file.getName(), decodeError);
                }
            }
            return metadata;
        } catch (IOException | JSONException e) {
            Log.w(TAG, "Unable to inspect bundle " + file.getName(), e);
            return null;
        } finally {
            if (tempCopy != null) {
                GameFileResolver.deleteRecursively(tempCopy);
            }
        }
    }

    private String formatFileSize(long bytes) {
        if (bytes < 1024) {
            return bytes + " B";
        }
        int exp = (int) (Math.log(bytes) / Math.log(1024));
        String pre = "KMGTPE".charAt(exp - 1) + "";
        return String.format(Locale.getDefault(), "%.1f %sB", bytes / Math.pow(1024, exp), pre);
    }

    private void filterGames(String query) {
        filteredGames.clear();

        if (query.trim().isEmpty()) {
            filteredGames.addAll(allGames);
        } else {
            String lowerQuery = query.toLowerCase(Locale.ROOT);
            for (GameInfo game : allGames) {
                if (game.getName().toLowerCase(Locale.ROOT).contains(lowerQuery)) {
                    filteredGames.add(game);
                }
            }
        }

        gameAdapter.notifyDataSetChanged();
        updateUI();
    }

    private void updateUI() {
        int gameCount = filteredGames.size();

        if (gameCount == 0) {
            gamesRecyclerView.setVisibility(View.GONE);
            emptyStateLayout.setVisibility(View.VISIBLE);

            if (allGames.isEmpty()) {
                gamesCountText.setText(R.string.no_games_found);
            } else {
                gamesCountText.setText("Nenhum jogo corresponde à pesquisa");
            }
        } else {
            gamesRecyclerView.setVisibility(View.VISIBLE);
            emptyStateLayout.setVisibility(View.GONE);
            gamesCountText.setText(getString(R.string.games_found, gameCount));
        }
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        getMenuInflater().inflate(R.menu.game_list_menu, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        int id = item.getItemId();

        if (id == android.R.id.home) {
            onBackPressed();
            return true;
        } else if (id == R.id.action_refresh) {
            scanForGames();
            return true;
        } else if (id == R.id.action_settings) {
            startActivity(new Intent(this, SettingsActivity.class));
            return true;
        } else if (id == R.id.action_about) {
            return true;
        }

        return super.onOptionsItemSelected(item);
    }

    @Override
    public void onGameClick(GameInfo gameInfo) {
        launchGame(gameInfo);
    }

    @Override
    public void onGameMenuClick(GameInfo gameInfo) {
        // Placeholder for future context actions.
    }

    private void launchGame(GameInfo gameInfo) {
        try {
            Intent intent = new Intent(this, MainActivity.class);
            intent.putExtra("game_uri", gameInfo.getFileUri().toString());
            intent.putExtra("game_name", gameInfo.getName());
            startActivity(intent);
        } catch (Exception e) {
            Log.e(TAG, "Failed to launch game", e);
        }
    }

    private static class NativeMetadata {
        String displayName;
        String version;
        String bundleIdentifier;
        Bitmap iconBitmap;
    }
}
