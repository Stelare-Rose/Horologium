package cc.starrytea.horologium.ui.nav

import androidx.navigation3.runtime.NavKey
import kotlinx.serialization.Serializable

@Serializable data object HomeRoute : NavKey

@Serializable data object SummaryRoute : NavKey

@Serializable data object ProjectsRoute : NavKey

@Serializable data object TagsRoute : NavKey

@Serializable data object SettingsRoute : NavKey

@Serializable data class RecordRoute(
    val id: String,
) : NavKey

data class TopLevel(
    val route: NavKey,
    val label: String,
)

val topLevel =
    listOf(
        TopLevel(HomeRoute, "Home"),
        TopLevel(SummaryRoute, "Summary"),
        TopLevel(ProjectsRoute, "Projects"),
        TopLevel(TagsRoute, "Tags"),
        TopLevel(SettingsRoute, "Settings"),
    )
