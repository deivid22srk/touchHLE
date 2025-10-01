/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.text.Editable;
import android.text.TextWatcher;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;
import androidx.documentfile.provider.DocumentFile;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;
import com.google.android.material.button.MaterialButton;
import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.google.android.material.textfield.TextInputEditText;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class GameListActivity extends AppCompatActivity implements GameAdapter.OnGameClickListener {

    private RecyclerView gamesRecyclerView;
    private TextView gamesCountText;
    private TextInputEditText searchEditText;
    private View emptyStateLayout;
    private MaterialButton changeFolderButton;
    private FloatingActionButton fileManagerFab;
    
    private GameAdapter gameAdapter;
    private List<GameInfo> allGames = new ArrayList<>();
    private List<GameInfo> filteredGames = new ArrayList<>();
    
    private Uri folderUri;
    private ExecutorService executorService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_game_list);

        initializeViews();
        setupRecyclerView();
        setupSearch();
        setupClickListeners();
        
        // Get folder URI from intent
        String folderUriString = getIntent().getStringExtra("folder_uri");
        if (folderUriString != null) {
            folderUri = Uri.parse(folderUriString);
            scanForGames();
        } else {
            // No folder selected, go back to folder selector
            startActivity(new Intent(this, FolderSelectorActivity.class));
            finish();
        }
    }

    private void initializeViews() {
        androidx.appcompat.widget.Toolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);
        
        gamesRecyclerView = findViewById(R.id.gamesRecyclerView);
        gamesCountText = findViewById(R.id.gamesCountText);
        searchEditText = findViewById(R.id.searchEditText);
        emptyStateLayout = findViewById(R.id.emptyStateLayout);
        changeFolderButton = findViewById(R.id.changeFolderButton);
        fileManagerFab = findViewById(R.id.fileManagerFab);
        
        // Enable back button
        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
        }
        
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
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {}

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {
                filterGames(s.toString());
            }

            @Override
            public void afterTextChanged(Editable s) {}
        });
    }

    private void setupClickListeners() {
        changeFolderButton.setOnClickListener(v -> {
            startActivity(new Intent(this, FolderSelectorActivity.class));
            finish();
        });

        fileManagerFab.setOnClickListener(v -> openFileManager());
        
        // Back button in toolbar
        findViewById(R.id.toolbar).setOnClickListener(v -> onBackPressed());
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
        if (files != null) {
            for (DocumentFile file : files) {
                if (file.isDirectory()) {
                    // Recursively scan subdirectories
                    scanDirectory(file, games);
                } else {
                    String fileName = file.getName();
                    if (fileName != null) {
                        if (fileName.toLowerCase().endsWith(".ipa") || 
                            fileName.toLowerCase().endsWith(".app")) {
                            GameInfo gameInfo = createGameInfo(file);
                            if (gameInfo != null) {
                                games.add(gameInfo);
                            }
                        }
                    }
                }
            }
        }
    }

    private GameInfo createGameInfo(DocumentFile file) {
        String fileName = file.getName();
        if (fileName == null) return null;
        
        // Extract name without extension
        String gameName = fileName;
        if (gameName.toLowerCase().endsWith(".ipa") || gameName.toLowerCase().endsWith(".app")) {
            gameName = gameName.substring(0, gameName.lastIndexOf('.'));
        }
        
        // Format file size
        long sizeBytes = file.length();
        String sizeString = formatFileSize(sizeBytes);
        
        return new GameInfo(
            gameName,
            "Unknown Version", // We'd need to parse the bundle for real version
            sizeString,
            file.getUri(),
            fileName.toLowerCase().endsWith(".ipa") ? GameInfo.Type.IPA : GameInfo.Type.APP
        );
    }

    private String formatFileSize(long bytes) {
        if (bytes < 1024) return bytes + " B";
        int exp = (int) (Math.log(bytes) / Math.log(1024));
        String pre = "KMGTPE".charAt(exp - 1) + "";
        return String.format("%.1f %sB", bytes / Math.pow(1024, exp), pre);
    }

    private void filterGames(String query) {
        filteredGames.clear();
        
        if (query.trim().isEmpty()) {
            filteredGames.addAll(allGames);
        } else {
            String lowerQuery = query.toLowerCase();
            for (GameInfo game : allGames) {
                if (game.getName().toLowerCase().contains(lowerQuery)) {
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

    private void openFileManager() {
        // Similar to FolderSelectorActivity's implementation
        try {
            Intent intent = new Intent(Intent.ACTION_VIEW);
            intent.setData(android.provider.DocumentsContract.buildRootsUri(getPackageName() + ".provider"));
            intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION |
                           Intent.FLAG_GRANT_WRITE_URI_PERMISSION);
            startActivity(intent);
        } catch (Exception e) {
            // Fallback: go to folder selector
            startActivity(new Intent(this, FolderSelectorActivity.class));
        }
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate menu programmatically since we removed menu reference from layout
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
            // TODO: Open settings
            return true;
        } else if (id == R.id.action_about) {
            // TODO: Open about dialog
            return true;
        }
        
        return super.onOptionsItemSelected(item);
    }

    @Override
    public void onGameClick(GameInfo gameInfo) {
        // Launch the game using the original MainActivity/SDL approach
        // This will need integration with the existing touchHLE core
        launchGame(gameInfo);
    }

    @Override
    public void onGameMenuClick(GameInfo gameInfo) {
        // TODO: Show context menu with options like:
        // - Game info
        // - Delete
        // - Move
        // - Properties
    }

    private void launchGame(GameInfo gameInfo) {
        // For now, we'll launch the original MainActivity with the game file
        // This needs integration with the existing touchHLE SDL code
        try {
            Intent intent = new Intent(this, MainActivity.class);
            intent.putExtra("game_uri", gameInfo.getFileUri().toString());
            intent.putExtra("game_name", gameInfo.getName());
            startActivity(intent);
        } catch (Exception e) {
            // Handle error launching game
            e.printStackTrace();
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        if (executorService != null) {
            executorService.shutdown();
        }
    }
}