/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.net.Uri;

public class GameInfo {
    
    public enum Type {
        IPA, APP
    }
    
    private String name;
    private String version;
    private String size;
    private Uri fileUri;
    private Type type;
    
    public GameInfo(String name, String version, String size, Uri fileUri, Type type) {
        this.name = name;
        this.version = version;
        this.size = size;
        this.fileUri = fileUri;
        this.type = type;
    }
    
    // Getters
    public String getName() {
        return name;
    }
    
    public String getVersion() {
        return version;
    }
    
    public String getSize() {
        return size;
    }
    
    public Uri getFileUri() {
        return fileUri;
    }
    
    public Type getType() {
        return type;
    }
    
    // Setters
    public void setName(String name) {
        this.name = name;
    }
    
    public void setVersion(String version) {
        this.version = version;
    }
    
    public void setSize(String size) {
        this.size = size;
    }
    
    public void setFileUri(Uri fileUri) {
        this.fileUri = fileUri;
    }
    
    public void setType(Type type) {
        this.type = type;
    }
    
    @Override
    public String toString() {
        return "GameInfo{" +
                "name='" + name + '\'' +
                ", version='" + version + '\'' +
                ", size='" + size + '\'' +
                ", type=" + type +
                '}';
    }
}