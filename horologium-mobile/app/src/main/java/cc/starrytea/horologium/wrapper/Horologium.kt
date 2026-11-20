package cc.starrytea.horologium.wrapper

import android.content.Context
import android.os.Environment
import cc.starrytea.horologium.Native
import java.io.File

object Horologium {
    private lateinit var basePath: String
    private lateinit var deviceId: String

    fun init(
        context: Context,
        deviceId: String,
    ) {
        basePath = File(Environment.getExternalStorageDirectory(), "Horologium").absolutePath
        this.deviceId = deviceId
    }

    fun start(name: String): String = Native.start(basePath, deviceId, name)
}
