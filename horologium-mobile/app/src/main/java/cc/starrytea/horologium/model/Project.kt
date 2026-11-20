package cc.starrytea.horologium.model
import java.time.Instant

enum class ProjectState { Active, Done }

data class Project(
    val id: String,
    val name: String,
    val colors: List<String>,
    val state: ProjectState,
    val completedDate: Instant? = null,
)
