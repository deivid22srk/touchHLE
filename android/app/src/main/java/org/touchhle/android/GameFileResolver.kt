package org.touchhle.android

import android.content.Context
import android.net.Uri
import android.os.ParcelFileDescriptor
import androidx.documentfile.provider.DocumentFile
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream
import java.util.Locale

object GameFileResolver {
    private const val CACHE_DIR = "runtime_games"

    data class Result(
        val path: String,
        val cacheFile: File?,
        val pfd: ParcelFileDescriptor?
    )

    @Throws(IOException::class)
    fun resolveForLaunch(context: Context, uri: Uri?): Result? {
        uri ?: return null

        val scheme = uri.scheme
        if ("file".equals(scheme, ignoreCase = true)) {
            val file = File(uri.path ?: return null)
            return if (file.exists()) {
                Result(file.absolutePath, null, null)
            } else null
        }

        if ("content".equals(scheme, ignoreCase = true)) {
            val documentFile = DocumentFile.fromSingleUri(context, uri) ?: return null
            if (!documentFile.exists()) return null

            val name = documentFile.name
            val isDir = documentFile.isDirectory
            val isIpa = name != null && !isDir && name.lowercase(Locale.ROOT).endsWith(".ipa")

            return if (isIpa) {
                val pfd = context.contentResolver.openFileDescriptor(documentFile.uri, "r")
                    ?: return null
                val fdPath = "/proc/self/fd/${pfd.fd}"
                Result(fdPath, null, pfd)
            } else {
                val cached = copyToCache(context, documentFile, "launch_")
                cached?.let { Result(it.absolutePath, it, null) }
            }
        }

        return null
    }

    @Throws(IOException::class)
    fun copyForInspection(context: Context, documentFile: DocumentFile): File? {
        return copyToCache(context, documentFile, "inspect_")
    }

    fun pruneCache(context: Context, maxAgeMs: Long, maxTotalBytes: Long) {
        val cacheRoot = File(context.cacheDir, CACHE_DIR)
        if (!cacheRoot.exists() || !cacheRoot.isDirectory) return

        var entries = cacheRoot.listFiles() ?: return
        val now = System.currentTimeMillis()

        // Age-based prune
        entries.forEach { f ->
            if (now - f.lastModified() > maxAgeMs) {
                deleteRecursively(f)
            }
        }

        // Size-based prune
        entries = cacheRoot.listFiles() ?: return
        var total = entries.sumOf { dirSize(it) }

        if (total > maxTotalBytes) {
            entries.sortBy { it.lastModified() }
            var i = 0
            while (total > maxTotalBytes && i < entries.size) {
                val sz = dirSize(entries[i])
                deleteRecursively(entries[i])
                total -= sz
                i++
            }
        }
    }

    private fun dirSize(f: File): Long {
        if (!f.exists()) return 0
        if (f.isFile) return f.length()

        var s = 0L
        f.listFiles()?.forEach { k ->
            s += dirSize(k)
        }
        return s
    }

    fun deleteRecursively(file: File?) {
        file ?: return
        if (!file.exists()) return

        if (file.isDirectory) {
            file.listFiles()?.forEach { child ->
                deleteRecursively(child)
            }
        }

        @Suppress("UNUSED_VARIABLE")
        val ignored = file.delete()
    }

    @Throws(IOException::class)
    private fun copyToCache(context: Context, documentFile: DocumentFile?, prefix: String): File? {
        documentFile ?: return null
        if (!documentFile.exists()) return null

        val cacheRoot = File(context.cacheDir, CACHE_DIR)
        if (!cacheRoot.exists() && !cacheRoot.mkdirs()) {
            throw IOException("Failed to create cache directory: ${cacheRoot.absolutePath}")
        }

        var name = documentFile.name
        if (name.isNullOrBlank()) {
            name = if (documentFile.isDirectory) "bundle" else "game"
        }
        val sanitized = sanitizeFileName(name)
        val timestamp = System.currentTimeMillis()

        return if (documentFile.isDirectory) {
            val targetDir = File(cacheRoot, "${prefix}${sanitized}_$timestamp")
            if (!targetDir.mkdirs() && !targetDir.isDirectory) {
                throw IOException("Failed to create directory: ${targetDir.absolutePath}")
            }
            copyDirectoryRecursive(context, documentFile, targetDir)
            targetDir
        } else {
            val targetName = appendSuffixKeepingExtension("$prefix$sanitized", "_$timestamp")
            val targetFile = File(cacheRoot, targetName)
            openStream(context, documentFile).use { input ->
                FileOutputStream(targetFile).use { output ->
                    copyStream(input, output)
                }
            }
            targetFile
        }
    }

    @Throws(IOException::class)
    private fun copyDirectoryRecursive(context: Context, source: DocumentFile, destination: File) {
        source.listFiles().forEach { child ->
            var childName = child.name
            if (childName.isNullOrBlank()) {
                childName = if (child.isDirectory) "dir" else "file"
            }
            val sanitized = sanitizeFileName(childName)

            if (child.isDirectory) {
                val childDir = File(destination, sanitized)
                if (!childDir.exists() && !childDir.mkdirs()) {
                    throw IOException("Failed to create directory: ${childDir.absolutePath}")
                }
                copyDirectoryRecursive(context, child, childDir)
            } else {
                val targetFile = File(destination, sanitized)
                openStream(context, child).use { input ->
                    FileOutputStream(targetFile).use { output ->
                        copyStream(input, output)
                    }
                }
            }
        }
    }

    @Throws(IOException::class)
    private fun openStream(context: Context, file: DocumentFile): InputStream {
        return context.contentResolver.openInputStream(file.uri)
            ?: throw IOException("Unable to open input stream for URI ${file.uri}")
    }

    @Throws(IOException::class)
    private fun copyStream(input: InputStream, output: OutputStream) {
        val buffer = ByteArray(8192)
        var bytesRead: Int
        while (input.read(buffer).also { bytesRead = it } != -1) {
            output.write(buffer, 0, bytesRead)
        }
        output.flush()
    }

    private fun sanitizeFileName(name: String): String {
        return name.replace(Regex("""[\\/:*?"<>|]"""), "_")
    }

    private fun appendSuffixKeepingExtension(name: String, suffix: String): String {
        val dot = name.lastIndexOf('.')
        return if (dot == -1) {
            name + suffix
        } else {
            val base = name.substring(0, dot)
            val extension = name.substring(dot)
            base + suffix + extension
        }
    }
}
