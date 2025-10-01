/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.PorterDuff;
import android.graphics.PorterDuffXfermode;
import android.graphics.Rect;
import android.graphics.RectF;
import android.net.Uri;
import android.os.Bundle;
import android.text.Editable;
import android.text.TextWatcher;
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
import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.google.android.material.textfield.TextInputEditText;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.zip.ZipEntry;
import java.util.zip.ZipInputStream;

public class GameListActivity extends AppCompatActivity implements GameAdapter.OnGameClickListener {

    private static final String TAG = "GameListActivity";

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

    private static class IconCandidate {
        final Bitmap bitmap;
        final int score;

        IconCandidate(Bitmap bitmap, int score) {
            this.bitmap = bitmap;
            this.score = score;
        }
    }

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
        MaterialToolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);

        gamesRecyclerView = findViewById(R.id.gamesRecyclerView);
        gamesCountText = findViewById(R.id.gamesCountText);
        searchEditText = findViewById(R.id.searchEditText);
        emptyStateLayout = findViewById(R.id.emptyStateLayout);
        changeFolderButton = findViewById(R.id.changeFolderButton);
        fileManagerFab = findViewById(R.id.fileManagerFab);

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
        
        GameInfo gameInfo = new GameInfo(
            gameName,
            "Unknown Version", // We'd need to parse the bundle for real version
            sizeString,
            file.getUri(),
            fileName.toLowerCase().endsWith(".ipa") ? GameInfo.Type.IPA : GameInfo.Type.APP
        );
        gameInfo.setIcon(loadGameIcon(file));
        return gameInfo;
    }

    private String formatFileSize(long bytes) {
        if (bytes < 1024) return bytes + " B";
        int exp = (int) (Math.log(bytes) / Math.log(1024));
        String pre = "KMGTPE".charAt(exp - 1) + "";
        return String.format("%.1f %sB", bytes / Math.pow(1024, exp), pre);
    }

    private Bitmap loadGameIcon(DocumentFile file) {
        String name = file.getName();
        if (name == null) {
            return null;
        }

        String lower = name.toLowerCase(Locale.ROOT);
        Bitmap icon = null;

        if (file.isDirectory() || lower.endsWith(".app")) {
            icon = loadIconFromAppBundle(file, 0, 2);
        } else if (lower.endsWith(".ipa")) {
            icon = loadIconFromIpa(file);
        }

        if (icon != null) {
            Bitmap styled = applyIconStyling(icon);
            if (styled != icon) {
                icon.recycle();
            }
            return styled;
        }
        return null;
    }

    private Bitmap loadIconFromIpa(DocumentFile file) {
        try (InputStream inputStream = getContentResolver().openInputStream(file.getUri());
             ZipInputStream zipInputStream = new ZipInputStream(inputStream)) {
            ZipEntry entry;
            IconCandidate best = null;
            byte[] buffer = new byte[8192];

            while ((entry = zipInputStream.getNextEntry()) != null) {
                try {
                    if (entry.isDirectory()) {
                        continue;
                    }

                    String entryName = entry.getName();
                    if (entryName == null) {
                        continue;
                    }

                    String lowerEntry = entryName.toLowerCase(Locale.ROOT);
                    if (!lowerEntry.contains(".app/")) {
                        continue;
                    }

                    boolean looksLikeIcon = lowerEntry.contains("appicon")
                            || lowerEntry.contains("icon")
                            || lowerEntry.contains("itunesartwork");
                    boolean supportedFormat = lowerEntry.endsWith(".png")
                            || lowerEntry.endsWith(".jpg")
                            || lowerEntry.endsWith(".jpeg");

                    if (!looksLikeIcon || !supportedFormat) {
                        continue;
                    }

                    ByteArrayOutputStream imageBytes = new ByteArrayOutputStream();
                    int read;
                    while ((read = zipInputStream.read(buffer)) != -1) {
                        imageBytes.write(buffer, 0, read);
                    }

                    IconCandidate candidate = decodeIconCandidate(imageBytes.toByteArray());
                    if (candidate != null) {
                        best = pickBetterIcon(best, candidate);
                    }
                } catch (OutOfMemoryError error) {
                    Log.w(TAG, "Icon too large inside " + entry.getName(), error);
                } finally {
                    zipInputStream.closeEntry();
                }
            }

            return best != null ? best.bitmap : null;
        } catch (IOException e) {
            Log.w(TAG, "Failed to load icon from " + file.getName(), e);
        }
        return null;
    }

    private Bitmap loadIconFromAppBundle(DocumentFile directory, int depth, int maxDepth) {
        if (!directory.isDirectory()) {
            return null;
        }
        IconCandidate best = traverseBundleForIcon(directory, depth, maxDepth, null);
        return best != null ? best.bitmap : null;
    }

    private IconCandidate traverseBundleForIcon(DocumentFile directory, int depth, int maxDepth, IconCandidate currentBest) {
        DocumentFile[] children = directory.listFiles();
        if (children == null) {
            return currentBest;
        }
        IconCandidate best = currentBest;
        for (DocumentFile child : children) {
            if (child.isDirectory()) {
                if (depth < maxDepth) {
                    best = traverseBundleForIcon(child, depth + 1, maxDepth, best);
                }
            } else {
                String childName = child.getName();
                if (childName == null) {
                    continue;
                }
                String lowerName = childName.toLowerCase(Locale.ROOT);
                boolean looksLikeIcon = lowerName.contains("appicon")
                        || lowerName.contains("icon")
                        || lowerName.contains("itunesartwork");
                boolean supportedFormat = lowerName.endsWith(".png")
                        || lowerName.endsWith(".jpg")
                        || lowerName.endsWith(".jpeg");
                if (!looksLikeIcon || !supportedFormat) {
                    continue;
                }
                IconCandidate candidate = decodeIconCandidate(child);
                if (candidate != null) {
                    best = pickBetterIcon(best, candidate);
                }
            }
        }
        return best;
    }

    private IconCandidate decodeIconCandidate(DocumentFile file) {
        try (InputStream inputStream = getContentResolver().openInputStream(file.getUri())) {
            if (inputStream == null) {
                return null;
            }
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            byte[] chunk = new byte[8192];
            int read;
            while ((read = inputStream.read(chunk)) != -1) {
                buffer.write(chunk, 0, read);
            }
            return decodeIconCandidate(buffer.toByteArray());
        } catch (IOException e) {
            Log.w(TAG, "Failed to decode icon file " + file.getName(), e);
        }
        return null;
    }

    private IconCandidate decodeIconCandidate(byte[] data) {
        BitmapFactory.Options bounds = new BitmapFactory.Options();
        bounds.inJustDecodeBounds = true;
        BitmapFactory.decodeByteArray(data, 0, data.length, bounds);
        int width = bounds.outWidth;
        int height = bounds.outHeight;
        if (width <= 0 || height <= 0) {
            return null;
        }

        BitmapFactory.Options decodeOptions = new BitmapFactory.Options();
        int maxDimension = Math.max(width, height);
        int sampleSize = 1;
        while (maxDimension / sampleSize > 1024) {
            sampleSize *= 2;
        }
        decodeOptions.inSampleSize = sampleSize;
        decodeOptions.inPreferredConfig = Bitmap.Config.ARGB_8888;

        Bitmap bitmap = BitmapFactory.decodeByteArray(data, 0, data.length, decodeOptions);
        if (bitmap == null) {
            return null;
        }
        return new IconCandidate(bitmap, width * height);
    }

    private IconCandidate pickBetterIcon(IconCandidate current, IconCandidate candidate) {
        if (candidate == null) {
            return current;
        }
        if (current == null || candidate.score > current.score) {
            if (current != null && current.bitmap != null && !current.bitmap.isRecycled()) {
                current.bitmap.recycle();
            }
            return candidate;
        } else {
            if (candidate.bitmap != null && !candidate.bitmap.isRecycled()) {
                candidate.bitmap.recycle();
            }
            return current;
        }
    }

    private Bitmap applyIconStyling(Bitmap source) {
        int width = source.getWidth();
        int height = source.getHeight();
        int size = Math.min(width, height);
        if (size <= 0) {
            return source;
        }

        Bitmap squared = Bitmap.createScaledBitmap(source, size, size, true);
        Bitmap output = Bitmap.createBitmap(size, size, Bitmap.Config.ARGB_8888);

        Canvas canvas = new Canvas(output);
        Paint paint = new Paint(Paint.ANTI_ALIAS_FLAG);
        Rect rect = new Rect(0, 0, size, size);
        RectF rectF = new RectF(rect);
        float radius = size * 0.23f;

        paint.setColor(Color.WHITE);
        canvas.drawRoundRect(rectF, radius, radius, paint);
        paint.setXfermode(new PorterDuffXfermode(PorterDuff.Mode.SRC_IN));
        canvas.drawBitmap(squared, null, rect, paint);
        paint.setXfermode(null);

        if (squared != source && !squared.isRecycled()) {
            squared.recycle();
        }

        return output;
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