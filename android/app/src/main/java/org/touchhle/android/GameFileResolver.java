package org.touchhle.android;

import android.content.Context;
import android.net.Uri;

import androidx.documentfile.provider.DocumentFile;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.Locale;

final class GameFileResolver {

    private static final String CACHE_DIR = "runtime_games";

    private GameFileResolver() {
    }

    static final class Result {
        private final String path;
        private final File cacheFile;

        Result(String path, File cacheFile) {
            this.path = path;
            this.cacheFile = cacheFile;
        }

        String getPath() {
            return path;
        }

        File getCacheFile() {
            return cacheFile;
        }
    }

    static Result resolveForLaunch(Context context, Uri uri) throws IOException {
        if (uri == null) {
            return null;
        }

        String scheme = uri.getScheme();
        if ("file".equalsIgnoreCase(scheme)) {
            File file = new File(uri.getPath());
            if (file.exists()) {
                return new Result(file.getAbsolutePath(), null);
            }
            return null;
        }

        if ("content".equalsIgnoreCase(scheme)) {
            DocumentFile documentFile = DocumentFile.fromSingleUri(context, uri);
            if (documentFile == null || !documentFile.exists()) {
                return null;
            }
            File cached = copyToCache(context, documentFile, "launch_");
            return cached != null ? new Result(cached.getAbsolutePath(), cached) : null;
        }

        return null;
    }

    static File copyForInspection(Context context, DocumentFile documentFile) throws IOException {
        return copyToCache(context, documentFile, "inspect_");
    }

    static void deleteRecursively(File file) {
        if (file == null || !file.exists()) {
            return;
        }
        if (file.isDirectory()) {
            File[] children = file.listFiles();
            if (children != null) {
                for (File child : children) {
                    deleteRecursively(child);
                }
            }
        }
        // ignore result; cache directory is best-effort cleanup
        //noinspection ResultOfMethodCallIgnored
        file.delete();
    }

    private static File copyToCache(Context context, DocumentFile documentFile, String prefix) throws IOException {
        if (documentFile == null || !documentFile.exists()) {
            return null;
        }
        File cacheRoot = new File(context.getCacheDir(), CACHE_DIR);
        if (!cacheRoot.exists() && !cacheRoot.mkdirs()) {
            throw new IOException("Failed to create cache directory: " + cacheRoot.getAbsolutePath());
        }

        String name = documentFile.getName();
        if (name == null || name.trim().isEmpty()) {
            name = documentFile.isDirectory() ? "bundle" : "game";
        }
        String sanitized = sanitizeFileName(name);
        long timestamp = System.currentTimeMillis();

        if (documentFile.isDirectory()) {
            File targetDir = new File(cacheRoot, prefix + sanitized + "_" + timestamp);
            if (!targetDir.mkdirs() && !targetDir.isDirectory()) {
                throw new IOException("Failed to create directory: " + targetDir.getAbsolutePath());
            }
            copyDirectoryRecursive(context, documentFile, targetDir);
            return targetDir;
        } else {
            String targetName = appendSuffixKeepingExtension(prefix + sanitized, "_" + timestamp);
            File targetFile = new File(cacheRoot, targetName);
            try (InputStream in = openStream(context, documentFile);
                 OutputStream out = new FileOutputStream(targetFile)) {
                copyStream(in, out);
            }
            return targetFile;
        }
    }

    private static void copyDirectoryRecursive(Context context, DocumentFile source, File destination) throws IOException {
        for (DocumentFile child : source.listFiles()) {
            String childName = child.getName();
            if (childName == null || childName.trim().isEmpty()) {
                childName = child.isDirectory() ? "dir" : "file";
            }
            String sanitized = sanitizeFileName(childName);
            if (child.isDirectory()) {
                File childDir = new File(destination, sanitized);
                if (!childDir.exists() && !childDir.mkdirs()) {
                    throw new IOException("Failed to create directory: " + childDir.getAbsolutePath());
                }
                copyDirectoryRecursive(context, child, childDir);
            } else {
                File targetFile = new File(destination, sanitized);
                try (InputStream in = openStream(context, child);
                     OutputStream out = new FileOutputStream(targetFile)) {
                    copyStream(in, out);
                }
            }
        }
    }

    private static InputStream openStream(Context context, DocumentFile file) throws IOException {
        InputStream inputStream = context.getContentResolver().openInputStream(file.getUri());
        if (inputStream == null) {
            throw new IOException("Unable to open input stream for URI " + file.getUri());
        }
        return inputStream;
    }

    private static void copyStream(InputStream in, OutputStream out) throws IOException {
        byte[] buffer = new byte[8192];
        int bytesRead;
        while ((bytesRead = in.read(buffer)) != -1) {
            out.write(buffer, 0, bytesRead);
        }
        out.flush();
    }

    private static String sanitizeFileName(String name) {
        return name.replaceAll("[\\\\/:*?\"<>|]", "_");
    }

    private static String appendSuffixKeepingExtension(String name, String suffix) {
        int dot = name.lastIndexOf('.');
        if (dot == -1) {
            return name + suffix;
        }
        String base = name.substring(0, dot);
        String extension = name.substring(dot);
        return base + suffix + extension;
    }
}
