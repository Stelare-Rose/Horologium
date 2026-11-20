package cc.starrytea.horologium.data

import cc.starrytea.horologium.model.Project
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.model.Tag

interface RecordStore {
    val records: List<Record>

    fun active(): Record?

    fun start(
        name: String,
        project: Project? = null,
        tags: List<Tag> = emptyList(),
    ): Record

    fun stop(id: String)
}
