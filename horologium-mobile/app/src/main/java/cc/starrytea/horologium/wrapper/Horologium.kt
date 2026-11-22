package cc.starrytea.horologium.wrapper

import android.content.Context
import android.os.Environment
import cc.starrytea.horologium.Native
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import java.io.File

object Horologium {
    private lateinit var basePath: String
    private lateinit var deviceId: String

    private val initLock = Mutex()
    private val _ready = MutableStateFlow(false)
    val ready: StateFlow<Boolean> = _ready

    suspend fun init(
        context: Context,
        deviceId: String,
    ) {
        initLock.withLock {
            if (_ready.value) return

            val base = File(Environment.getExternalStorageDirectory(), "Horologium").absolutePath
            val dbPath = File(context.filesDir, "horologium.db").absolutePath

            withContext(Dispatchers.IO) {
                Native.init(base, dbPath)
            }

            basePath = base
            this.deviceId = deviceId
            _ready.value = true
        }
    }

    fun start(name: String): String = Native.start(basePath, deviceId, name)
}
