package cc.starrytea.horologium

object Native {
    init {
        System.loadLibrary("horologium_lib_android")
    }

    external fun start(
        basePath: String,
        deviceId: String,
        name: String,
    ): String

    external fun init(
        basePath: String,
        dbPath: String,
    )
}
