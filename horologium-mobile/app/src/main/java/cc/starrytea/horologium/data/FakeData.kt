package cc.starrytea.horologium.data

import cc.starrytea.horologium.model.Project
import cc.starrytea.horologium.model.ProjectState
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.model.RecordStatus
import cc.starrytea.horologium.model.Tag
import java.time.Instant

object FakeData {
    val thesis = Project("proj_a1b2c3d4e5f", "Thesis", listOf("#cf8282"), ProjectState.Active)
    val deep = Tag("tag_x9y8z7w6v5u", "deep work", listOf("#9bb0de"))
    val admin = Tag("tag_q1w2e3r4t5y", "admin", listOf("#e3cd7f"))

    val records =
        listOf(
            Record(
                id = "0193...a1",
                name = "Writing chapter 2",
                status = RecordStatus.Inactive,
                start = Instant.parse("2026-11-20T01:41:00Z"),
                end = Instant.parse("2026-11-20T03:22:00Z"),
                project = thesis,
                tags = listOf(deep),
            ),
            Record(
                id = "0193...a2",
                name = "Emails",
                status = RecordStatus.Inactive,
                start = Instant.parse("2026-11-20T03:25:00Z"),
                end = Instant.parse("2026-11-20T03:47:00Z"),
                tags = listOf(admin),
            ),
            Record(
                id = "0193...a3",
                name = "Reading papers",
                status = RecordStatus.Active,
                start = Instant.parse("2026-11-20T03:50:00Z"),
                project = thesis,
                tags = listOf(deep),
            ),
        )
}
