package org.touchhle.android;

import android.os.Bundle;
import android.widget.RadioGroup;

import androidx.appcompat.app.AppCompatActivity;

import com.google.android.material.appbar.MaterialToolbar;
import com.google.android.material.button.MaterialButton;
import com.google.android.material.switchmaterial.SwitchMaterial;

public class SettingsActivity extends AppCompatActivity {
    private RadioGroup scaleHackGroup;
    private RadioGroup orientationGroup;
    private SwitchMaterial analogSwitch;
    private SwitchMaterial networkSwitch;
    private SwitchMaterial fullscreenSwitch;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_settings);

        MaterialToolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);
        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
        }
        toolbar.setNavigationOnClickListener(v -> getOnBackPressedDispatcher().onBackPressed());

        scaleHackGroup = findViewById(R.id.scaleHackGroup);
        orientationGroup = findViewById(R.id.orientationGroup);
        analogSwitch = findViewById(R.id.analogStickSwitch);
        networkSwitch = findViewById(R.id.networkSwitch);
        fullscreenSwitch = findViewById(R.id.fullscreenSwitch);
        MaterialButton saveButton = findViewById(R.id.saveButton);

        applySavedValues();
        saveButton.setOnClickListener(v -> saveAndClose());
    }

    private void applySavedValues() {
        int scaleHack = SettingsManager.getScaleHack(this);
        checkScaleHack(scaleHack);

        int orientation = SettingsManager.getOrientation(this);
        checkOrientation(orientation);

        analogSwitch.setChecked(SettingsManager.getAnalog(this));
        networkSwitch.setChecked(SettingsManager.getNetwork(this));
        fullscreenSwitch.setChecked(SettingsManager.getFullscreen(this));
    }

    private void checkScaleHack(int value) {
        int buttonId;
        if (value == SettingsManager.SCALE_OFF) {
            buttonId = R.id.scaleHackOff;
        } else if (value == SettingsManager.SCALE_TWO) {
            buttonId = R.id.scaleHackTwo;
        } else if (value == SettingsManager.SCALE_THREE) {
            buttonId = R.id.scaleHackThree;
        } else if (value == SettingsManager.SCALE_FOUR) {
            buttonId = R.id.scaleHackFour;
        } else {
            buttonId = R.id.scaleHackDefault;
        }
        scaleHackGroup.check(buttonId);
    }

    private void checkOrientation(int value) {
        int buttonId;
        if (value == SettingsManager.ORIENTATION_LEFT) {
            buttonId = R.id.orientationLeft;
        } else if (value == SettingsManager.ORIENTATION_RIGHT) {
            buttonId = R.id.orientationRight;
        } else {
            buttonId = R.id.orientationDefault;
        }
        orientationGroup.check(buttonId);
    }

    private void saveAndClose() {
        int scaleHack = resolveScaleHack();
        int orientation = resolveOrientation();
        boolean analog = analogSwitch.isChecked();
        boolean network = networkSwitch.isChecked();
        boolean fullscreen = fullscreenSwitch.isChecked();

        SettingsManager.saveAll(this, scaleHack, orientation, analog, network, fullscreen);
        finish();
    }

    private int resolveScaleHack() {
        int checked = scaleHackGroup.getCheckedRadioButtonId();
        if (checked == R.id.scaleHackOff) {
            return SettingsManager.SCALE_OFF;
        } else if (checked == R.id.scaleHackTwo) {
            return SettingsManager.SCALE_TWO;
        } else if (checked == R.id.scaleHackThree) {
            return SettingsManager.SCALE_THREE;
        } else if (checked == R.id.scaleHackFour) {
            return SettingsManager.SCALE_FOUR;
        }
        return SettingsManager.SCALE_DEFAULT;
    }

    private int resolveOrientation() {
        int checked = orientationGroup.getCheckedRadioButtonId();
        if (checked == R.id.orientationLeft) {
            return SettingsManager.ORIENTATION_LEFT;
        } else if (checked == R.id.orientationRight) {
            return SettingsManager.ORIENTATION_RIGHT;
        }
        return SettingsManager.ORIENTATION_DEFAULT;
    }
}
