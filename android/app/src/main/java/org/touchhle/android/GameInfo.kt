/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android

import android.graphics.Bitmap
import android.net.Uri

data class GameInfo(
    var name: String,
    var version: String,
    var size: String,
    val fileUri: Uri,
    val type: Type
) {
    enum class Type {
        IPA, APP
    }
    
    var icon: Bitmap? = null
    var bundleIdentifier: String? = null
    
    override fun toString(): String {
        return "GameInfo(name='$name', version='$version', size='$size', type=$type)"
    }
}
