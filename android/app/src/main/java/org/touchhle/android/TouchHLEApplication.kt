package org.touchhle.android

import android.app.Application
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.util.Log
import java.io.PrintWriter
import java.io.StringWriter

class TouchHLEApplication : Application() {
    companion object {
        private const val TAG = "TouchHLEApplication"
    }

    override fun onCreate() {
        super.onCreate()
        setupGlobalCrashHandler()
    }

    private fun setupGlobalCrashHandler() {
        val defaultHandler = Thread.getDefaultUncaughtExceptionHandler()
        Thread.setDefaultUncaughtExceptionHandler { thread, throwable ->
            Log.e(TAG, "Uncaught exception caught", throwable)

            if (SettingsManager.getAutoCopyError(this)) {
                try {
                    val sw = StringWriter()
                    val pw = PrintWriter(sw)
                    throwable.printStackTrace(pw)
                    val stackTrace = sw.toString()

                    val clipboard = getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                    val clip = ClipData.newPlainText("touchHLE Crash Log", stackTrace)
                    clipboard.setPrimaryClip(clip)
                    Log.i(TAG, "Crash log copied to clipboard.")
                } catch (e: Exception) {
                    Log.e(TAG, "Failed to copy crash log to clipboard.", e)
                }
            }

            defaultHandler?.uncaughtException(thread, throwable)
        }
    }
}
