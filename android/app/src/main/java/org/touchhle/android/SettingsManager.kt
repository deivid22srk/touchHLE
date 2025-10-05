package org.touchhle.android

import android.content.Context
import android.content.SharedPreferences

object SettingsManager {
    const val SCALE_DEFAULT = 0
    const val SCALE_OFF = 1
    const val SCALE_TWO = 2
    const val SCALE_THREE = 3
    const val SCALE_FOUR = 4

    const val ORIENTATION_DEFAULT = 0
    const val ORIENTATION_LEFT = 1
    const val ORIENTATION_RIGHT = 2

    private const val PREFS_NAME = "touchhle_settings"
    private const val KEY_SCALE_HACK = "scale_hack"
    private const val KEY_ORIENTATION = "orientation"
    private const val KEY_ANALOG = "analog"
    private const val KEY_NETWORK = "network"
    private const val KEY_FULLSCREEN = "fullscreen"
    private const val KEY_AUTO_COPY_ERROR = "auto_copy_error"

    private fun prefs(context: Context): SharedPreferences {
        return context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
    }

    fun getScaleHack(context: Context): Int {
        return prefs(context).getInt(KEY_SCALE_HACK, SCALE_DEFAULT)
    }

    fun getOrientation(context: Context): Int {
        return prefs(context).getInt(KEY_ORIENTATION, ORIENTATION_DEFAULT)
    }

    fun getAnalog(context: Context): Boolean {
        return prefs(context).getBoolean(KEY_ANALOG, true)
    }

    fun getNetwork(context: Context): Boolean {
        return prefs(context).getBoolean(KEY_NETWORK, false)
    }

    fun getFullscreen(context: Context): Boolean {
        return prefs(context).getBoolean(KEY_FULLSCREEN, false)
    }

    fun getAutoCopyError(context: Context): Boolean {
        return prefs(context).getBoolean(KEY_AUTO_COPY_ERROR, false)
    }

    fun saveAll(
        context: Context,
        scale: Int,
        orientation: Int,
        analog: Boolean,
        network: Boolean,
        fullscreen: Boolean,
        autoCopyError: Boolean
    ) {
        prefs(context).edit()
            .putInt(KEY_SCALE_HACK, scale)
            .putInt(KEY_ORIENTATION, orientation)
            .putBoolean(KEY_ANALOG, analog)
            .putBoolean(KEY_NETWORK, network)
            .putBoolean(KEY_FULLSCREEN, fullscreen)
            .putBoolean(KEY_AUTO_COPY_ERROR, autoCopyError)
            .apply()
    }

    fun buildOptionArgs(context: Context): Array<String> {
        val preferences = prefs(context)
        val args = mutableListOf<String>()

        val scale = preferences.getInt(KEY_SCALE_HACK, SCALE_DEFAULT)
        if (scale != SCALE_DEFAULT) {
            args.add("--scale-hack=$scale")
        }

        when (preferences.getInt(KEY_ORIENTATION, ORIENTATION_DEFAULT)) {
            ORIENTATION_LEFT -> args.add("--landscape-left")
            ORIENTATION_RIGHT -> args.add("--landscape-right")
        }

        if (!preferences.getBoolean(KEY_ANALOG, true)) {
            args.add("--disable-analog-stick-tilt-controls")
        }

        if (preferences.getBoolean(KEY_NETWORK, false)) {
            args.add("--allow-network-access")
        }

        if (preferences.getBoolean(KEY_FULLSCREEN, false)) {
            args.add("--fullscreen")
        }

        return args.toTypedArray()
    }
}
