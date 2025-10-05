package org.touchhle.android

import android.os.Bundle
import android.widget.RadioGroup
import androidx.appcompat.app.AppCompatActivity
import com.google.android.material.appbar.MaterialToolbar
import com.google.android.material.button.MaterialButton
import com.google.android.material.switchmaterial.SwitchMaterial

class SettingsActivity : AppCompatActivity() {
    private lateinit var scaleHackGroup: RadioGroup
    private lateinit var orientationGroup: RadioGroup
    private lateinit var analogSwitch: SwitchMaterial
    private lateinit var networkSwitch: SwitchMaterial
    private lateinit var fullscreenSwitch: SwitchMaterial
    private lateinit var autoCopyErrorSwitch: SwitchMaterial

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_settings)

        val toolbar = findViewById<MaterialToolbar>(R.id.toolbar)
        setSupportActionBar(toolbar)
        supportActionBar?.setDisplayHomeAsUpEnabled(true)
        toolbar.setNavigationOnClickListener {
            onBackPressedDispatcher.onBackPressed()
        }

        scaleHackGroup = findViewById(R.id.scaleHackGroup)
        orientationGroup = findViewById(R.id.orientationGroup)
        analogSwitch = findViewById(R.id.analogStickSwitch)
        networkSwitch = findViewById(R.id.networkSwitch)
        fullscreenSwitch = findViewById(R.id.fullscreenSwitch)
        autoCopyErrorSwitch = findViewById(R.id.autoCopyErrorSwitch)
        val saveButton = findViewById<MaterialButton>(R.id.saveButton)

        applySavedValues()
        setupAutoSave()
        saveButton.setOnClickListener { finish() }
    }

    private fun applySavedValues() {
        val scaleHack = SettingsManager.getScaleHack(this)
        checkScaleHack(scaleHack)

        val orientation = SettingsManager.getOrientation(this)
        checkOrientation(orientation)

        analogSwitch.isChecked = SettingsManager.getAnalog(this)
        networkSwitch.isChecked = SettingsManager.getNetwork(this)
        fullscreenSwitch.isChecked = SettingsManager.getFullscreen(this)
        autoCopyErrorSwitch.isChecked = SettingsManager.getAutoCopyError(this)
    }

    private fun checkScaleHack(value: Int) {
        val buttonId = when (value) {
            SettingsManager.SCALE_OFF -> R.id.scaleHackOff
            SettingsManager.SCALE_TWO -> R.id.scaleHackTwo
            SettingsManager.SCALE_THREE -> R.id.scaleHackThree
            SettingsManager.SCALE_FOUR -> R.id.scaleHackFour
            else -> R.id.scaleHackDefault
        }
        scaleHackGroup.check(buttonId)
    }

    private fun checkOrientation(value: Int) {
        val buttonId = when (value) {
            SettingsManager.ORIENTATION_LEFT -> R.id.orientationLeft
            SettingsManager.ORIENTATION_RIGHT -> R.id.orientationRight
            else -> R.id.orientationDefault
        }
        orientationGroup.check(buttonId)
    }

    private fun resolveScaleHack(): Int {
        return when (scaleHackGroup.checkedRadioButtonId) {
            R.id.scaleHackOff -> SettingsManager.SCALE_OFF
            R.id.scaleHackTwo -> SettingsManager.SCALE_TWO
            R.id.scaleHackThree -> SettingsManager.SCALE_THREE
            R.id.scaleHackFour -> SettingsManager.SCALE_FOUR
            else -> SettingsManager.SCALE_DEFAULT
        }
    }

    private fun resolveOrientation(): Int {
        return when (orientationGroup.checkedRadioButtonId) {
            R.id.orientationLeft -> SettingsManager.ORIENTATION_LEFT
            R.id.orientationRight -> SettingsManager.ORIENTATION_RIGHT
            else -> SettingsManager.ORIENTATION_DEFAULT
        }
    }

    private fun setupAutoSave() {
        val switchListener = { _: android.widget.CompoundButton, _: Boolean ->
            saveSettings()
        }
        val radioListener = RadioGroup.OnCheckedChangeListener { _, _ ->
            saveSettings()
        }

        analogSwitch.setOnCheckedChangeListener(switchListener)
        networkSwitch.setOnCheckedChangeListener(switchListener)
        fullscreenSwitch.setOnCheckedChangeListener(switchListener)
        autoCopyErrorSwitch.setOnCheckedChangeListener(switchListener)
        scaleHackGroup.setOnCheckedChangeListener(radioListener)
        orientationGroup.setOnCheckedChangeListener(radioListener)
    }

    private fun saveSettings() {
        val scaleHack = resolveScaleHack()
        val orientation = resolveOrientation()
        val analog = analogSwitch.isChecked
        val network = networkSwitch.isChecked
        val fullscreen = fullscreenSwitch.isChecked
        val autoCopyError = autoCopyErrorSwitch.isChecked

        SettingsManager.saveAll(this, scaleHack, orientation, analog, network, fullscreen, autoCopyError)
    }
}
