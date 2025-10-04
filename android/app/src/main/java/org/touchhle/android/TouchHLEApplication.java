package org.touchhle.android;

import android.app.Application;
import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.Context;
import android.util.Log;

import java.io.PrintWriter;
import java.io.StringWriter;

public class TouchHLEApplication extends Application {
    private static final String TAG = "TouchHLEApplication";

    @Override
    public void onCreate() {
        super.onCreate();
        setupGlobalCrashHandler();
    }

    private void setupGlobalCrashHandler() {
        final Thread.UncaughtExceptionHandler defaultHandler = Thread.getDefaultUncaughtExceptionHandler();
        Thread.setDefaultUncaughtExceptionHandler((thread, throwable) -> {
            Log.e(TAG, "Uncaught exception caught", throwable);

            if (SettingsManager.getAutoCopyError(this)) {
                try {
                    StringWriter sw = new StringWriter();
                    PrintWriter pw = new PrintWriter(sw);
                    throwable.printStackTrace(pw);
                    String stackTrace = sw.toString();

                    ClipboardManager clipboard = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
                    ClipData clip = ClipData.newPlainText("touchHLE Crash Log", stackTrace);
                    clipboard.setPrimaryClip(clip);
                    Log.i(TAG, "Crash log copied to clipboard.");
                } catch (Exception e) {
                    Log.e(TAG, "Failed to copy crash log to clipboard.", e);
                }
            }

            if (defaultHandler != null) {
                defaultHandler.uncaughtException(thread, throwable);
            }
        });
    }
}