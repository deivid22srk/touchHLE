package org.touchhle.android

object TouchHLENative {
    init {
        System.loadLibrary("touchHLE")
    }

    external fun prepareLaunch(gamePath: String, gameName: String, optionArgs: Array<String>)

    external fun clearLaunch()

    external fun inspectBundle(absolutePath: String): String?
    
    external fun getCurrentFps(): Int
}
