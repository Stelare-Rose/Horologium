package cc.starrytea.horologium.wrapper
import android.content.Context
import android.os.Build
import android.provider.Settings
import java.io.File
import java.util.UUID

fun loadOrCreateDeviceId(context: Context): String {
    val file = File(context.filesDir, "device-id")
    if (file.exists()) return file.readText().trim()

    val rawName =
        Settings.Global
            .getString(context.contentResolver, "device_name")
            ?.takeIf { it.isNotBlank() }
            ?: Build.MODEL

    val name =
        rawName
            .lowercase()
            .replace(Regex("[^a-z0-9]+"), "-")
            .trim('-')
            .ifEmpty { "android" }

    val id = "$name-${UUID.randomUUID().toString().take(8)}"
    file.writeText(id)
    return id
}
