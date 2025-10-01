/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.provider.DocumentsContract;
import android.view.View;
import android.widget.TextView;

import androidx.activity.result.ActivityResultLauncher;
import androidx.activity.result.contract.ActivityResultContracts;
import androidx.appcompat.app.AppCompatActivity;
import androidx.documentfile.provider.DocumentFile;

import com.google.android.material.button.MaterialButton;

public class FolderSelectorActivity extends AppCompatActivity {

    private static final String PREF_FOLDER_URI = "folder_uri";
    private static final int REQUEST_CODE_FOLDER_PICKER = 1001;
    
    private TextView selectedFolderText;
    private MaterialButton continueButton;
    private MaterialButton browseButton;
    private MaterialButton fileManagerButton;
    
    private Uri selectedFolderUri;

    private ActivityResultLauncher<Intent> folderPickerLauncher = registerForActivityResult(
        new ActivityResultContracts.StartActivityForResult(),
        result -> {
            if (result.getResultCode() == Activity.RESULT_OK && result.getData() != null) {
                Uri uri = result.getData().getData();
                if (uri != null) {
                    // Take persistable permission
                    getContentResolver().takePersistableUriPermission(
                        uri,
                        Intent.FLAG_GRANT_READ_URI_PERMISSION | Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                    );
                    
                    setSelectedFolder(uri);
                }
            }
        }
    );

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_folder_selector);

        initializeViews();
        setupClickListeners();
        
        // Load previously selected folder if any
        loadSavedFolder();
    }

    private void initializeViews() {
        selectedFolderText = findViewById(R.id.selectedFolderText);
        continueButton = findViewById(R.id.continueButton);
        browseButton = findViewById(R.id.browseButton);
        fileManagerButton = findViewById(R.id.fileManagerButton);
    }

    private void setupClickListeners() {
        browseButton.setOnClickListener(v -> openFolderPicker());
        
        fileManagerButton.setOnClickListener(v -> openFileManager());
        
        continueButton.setOnClickListener(v -> {
            if (selectedFolderUri != null) {
                Intent intent = new Intent(this, GameListActivity.class);
                intent.putExtra("folder_uri", selectedFolderUri.toString());
                startActivity(intent);
                finish();
            }
        });
    }

    private void openFolderPicker() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
        intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION |
                       Intent.FLAG_GRANT_WRITE_URI_PERMISSION |
                       Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION |
                       Intent.FLAG_GRANT_PREFIX_URI_PERMISSION);
        
        folderPickerLauncher.launch(intent);
    }

    private void openFileManager() {
        // Try to open the touchHLE file manager via DocumentsProvider
        try {
            Intent intent = new Intent(Intent.ACTION_VIEW);
            intent.setData(DocumentsContract.buildRootsUri(getPackageName() + ".provider"));
            intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION |
                           Intent.FLAG_GRANT_WRITE_URI_PERMISSION);
            startActivity(intent);
        } catch (Exception e) {
            // Fallback: try to open any file manager
            try {
                Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
                folderPickerLauncher.launch(intent);
            } catch (Exception ex) {
                // Last resort: show a simple message
                // Could show a Snackbar here
            }
        }
    }

    private void setSelectedFolder(Uri folderUri) {
        selectedFolderUri = folderUri;
        
        // Update UI
        DocumentFile folder = DocumentFile.fromTreeUri(this, folderUri);
        if (folder != null) {
            String folderName = folder.getName();
            if (folderName == null || folderName.isEmpty()) {
                folderName = "Root Directory";
            }
            selectedFolderText.setText(getString(R.string.folder_selected, folderName));
            continueButton.setEnabled(true);
            
            // Save the folder URI
            getSharedPreferences("touchhle_prefs", MODE_PRIVATE)
                .edit()
                .putString(PREF_FOLDER_URI, folderUri.toString())
                .apply();
        }
    }

    private void loadSavedFolder() {
        String savedUriString = getSharedPreferences("touchhle_prefs", MODE_PRIVATE)
            .getString(PREF_FOLDER_URI, null);
        
        if (savedUriString != null) {
            try {
                Uri savedUri = Uri.parse(savedUriString);
                // Check if we still have permission
                DocumentFile folder = DocumentFile.fromTreeUri(this, savedUri);
                if (folder != null && folder.exists()) {
                    setSelectedFolder(savedUri);
                }
            } catch (Exception e) {
                // Invalid saved URI, ignore
            }
        }
    }
}