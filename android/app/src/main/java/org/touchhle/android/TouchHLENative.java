/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

/**
 * JNI interface to communicate with the native touchHLE core
 */
public class TouchHLENative {

    // Load the native library
    static {
        System.loadLibrary("touchHLE");
    }

    /**
     * Set the game path before starting the emulation
     * @param gamePath Path to the game file (from content URI)
     * @param gameName Display name of the game
     */
    public static native void setGamePath(String gamePath, String gameName);

    /**
     * Check if a game path has been set
     * @return true if a game path has been set
     */
    public static native boolean hasGamePath();

    /**
     * Get the currently set game path
     * @return the game path or null if none set
     */
    public static native String getGamePath();

    /**
     * Get the currently set game name
     * @return the game name or null if none set
     */
    public static native String getGameName();

    /**
     * Clear the game path
     */
    public static native void clearGamePath();
}