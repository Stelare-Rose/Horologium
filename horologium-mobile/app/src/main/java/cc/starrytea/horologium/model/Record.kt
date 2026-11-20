package cc.starrytea.horologium.model
import java.time.Instant

enum class RecordStatus { Active, Inactive }

data class Record(
    val id: String,
    val name: String,
    val status: RecordStatus,
    val start: Instant,
    val end: Instant? = null,
    val project: Project? = null,
    val tags: List<Tag> = emptyList(),
)
