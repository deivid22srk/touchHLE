package org.touchhle.android;

public final class TouchHLENative {
    static {
        System.loadLibrary("touchHLE");
    }

    private TouchHLENative() {
    }

    public static native void prepareLaunch(String gamePath, String gameName, String[] optionArgs);

    public static native void clearLaunch();

    public static native String inspectBundle(String absolutePath);
    
    public static native int getCurrentFps();
}
