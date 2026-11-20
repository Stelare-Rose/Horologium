package cc.starrytea.horologium.data

import android.content.Context
import android.database.sqlite.SQLiteDatabase
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.model.RecordStatus
import cc.starrytea.horologium.model.Tag
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.withContext
import java.io.File
import java.time.Instant

object Store {
    private lateinit var dbFile: File
    private var db: SQLiteDatabase? = null

    private val _active = MutableStateFlow<Record?>(null)
    val active: StateFlow<Record?> = _active

    fun init(context: Context) {
        dbFile = File(context.filesDir, "main.db")
    }

    @Synchronized
    private fun db(): SQLiteDatabase {
        db?.let { if (it.isOpen) return it }
        return SQLiteDatabase
            .openDatabase(dbFile.absolutePath, null, SQLiteDatabase.OPEN_READONLY)
            .also { db = it }
    }

    suspend fun refresh() =
        withContext(Dispatchers.IO) {
            _active.value = queryActive()
        }

    private fun queryActive(): Record? =
        Record(
            id = "mock-active",
            name = "Writing chapter 2",
            status = RecordStatus.Active,
            start = Instant.now().minusSeconds(37 * 60 + 12),
            tags =
                listOf(
                    Tag("tag_deepwork01", "deep work", listOf("#9bb0de")),
                    Tag("tag_thesis0001", "thesis", listOf("#cca3d6")),
                ),
        )

    suspend fun recordCount(): Int =
        withContext(Dispatchers.IO) {
            db().rawQuery("SELECT COUNT(*) FROM records", null).use {
                it.moveToFirst()
                it.getInt(0)
            }
        }

    fun close() {
        db?.close()
        db = null
    }
}
