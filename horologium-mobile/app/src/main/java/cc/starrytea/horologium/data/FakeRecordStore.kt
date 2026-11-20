package cc.starrytea.horologium.data

import androidx.compose.runtime.mutableStateListOf
import cc.starrytea.horologium.model.Project
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.model.RecordStatus
import cc.starrytea.horologium.model.Tag
import java.time.Instant
import java.util.UUID

class FakeRecordStore(
    initial: List<Record> = FakeData.records,
) : RecordStore {
    private val _records = mutableStateListOf<Record>().apply { addAll(initial) }

    override val records: List<Record> get() = _records

    override fun active(): Record? = _records.firstOrNull { it.status == RecordStatus.Active }

    override fun start(
        name: String,
        project: Project?,
        tags: List<Tag>,
    ): Record {
        active()?.let { stop(it.id) }
        val record =
            Record(
                id = UUID.randomUUID().toString(),
                name = name,
                status = RecordStatus.Active,
                start = Instant.now(),
                project = project,
                tags = tags,
            )
        _records.add(record)
        return record
    }

    override fun stop(id: String) {
        val index = _records.indexOfFirst { it.id == id }
        if (index == -1) return
        _records[index] = _records[index].copy(status = RecordStatus.Inactive, end = Instant.now())
    }
}
