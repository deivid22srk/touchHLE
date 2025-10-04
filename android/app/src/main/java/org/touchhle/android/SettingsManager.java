package org.touchhle.android;

import android.content.Context;
import android.content.SharedPreferences;

import java.util.ArrayList;
import java.util.List;

public final class SettingsManager {
    public static final int SCALE_DEFAULT = 0;
    public static final int SCALE_OFF = 1;
    public static final int SCALE_TWO = 2;
    public static final int SCALE_THREE = 3;
    public static final int SCALE_FOUR = 4;

    public static final int ORIENTATION_DEFAULT = 0;
    public static final int ORIENTATION_LEFT = 1;
    public static final int ORIENTATION_RIGHT = 2;

    private static final String PREFS_NAME = "touchhle_settings";
    private static final String KEY_SCALE_HACK = "scale_hack";
    private static final String KEY_ORIENTATION = "orientation";
    private static final String KEY_ANALOG = "analog";
    private static final String KEY_NETWORK = "network";
    private static final String KEY_FULLSCREEN = "fullscreen";
    private static final String KEY_AUTO_COPY_ERROR = "auto_copy_error";

    private SettingsManager() {
    }

    private static SharedPreferences prefs(Context context) {
        return context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
    }

    public static int getScaleHack(Context context) {
        return prefs(context).getInt(KEY_SCALE_HACK, SCALE_DEFAULT);
    }

    public static int getOrientation(Context context) {
        return prefs(context).getInt(KEY_ORIENTATION, ORIENTATION_DEFAULT);
    }

    public static boolean getAnalog(Context context) {
        return prefs(context).getBoolean(KEY_ANALOG, true);
    }

    public static boolean getNetwork(Context context) {
        return prefs(context).getBoolean(KEY_NETWORK, false);
    }

    public static boolean getFullscreen(Context context) {
        return prefs(context).getBoolean(KEY_FULLSCREEN, false);
    }

    public static boolean getAutoCopyError(Context context) {
        return prefs(context).getBoolean(KEY_AUTO_COPY_ERROR, false);
    }

    public static void saveAll(Context context, int scale, int orientation, boolean analog, boolean network, boolean fullscreen, boolean autoCopyError) {
        prefs(context).edit()
                .putInt(KEY_SCALE_HACK, scale)
                .putInt(KEY_ORIENTATION, orientation)
                .putBoolean(KEY_ANALOG, analog)
                .putBoolean(KEY_NETWORK, network)
                .putBoolean(KEY_FULLSCREEN, fullscreen)
                .putBoolean(KEY_AUTO_COPY_ERROR, autoCopyError)
                .apply();
    }

    public static String[] buildOptionArgs(Context context) {
        SharedPreferences preferences = prefs(context);
        List<String> args = new ArrayList<>();

        int scale = preferences.getInt(KEY_SCALE_HACK, SCALE_DEFAULT);
        if (scale != SCALE_DEFAULT) {
            args.add("--scale-hack=" + scale);
        }

        int orientation = preferences.getInt(KEY_ORIENTATION, ORIENTATION_DEFAULT);
        if (orientation == ORIENTATION_LEFT) {
            args.add("--landscape-left");
        } else if (orientation == ORIENTATION_RIGHT) {
            args.add("--landscape-right");
        }

        if (!preferences.getBoolean(KEY_ANALOG, true)) {
            args.add("--disable-analog-stick-tilt-controls");
        }

        if (preferences.getBoolean(KEY_NETWORK, false)) {
            args.add("--allow-network-access");
        }

        if (preferences.getBoolean(KEY_FULLSCREEN, false)) {
            args.add("--fullscreen");
        }

        return args.toArray(new String[0]);
    }
}
