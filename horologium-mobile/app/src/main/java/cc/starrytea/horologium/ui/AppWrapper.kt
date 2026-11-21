@file:Suppress("FunctionName")

package cc.starrytea.horologium.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material3.DrawerValue
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalDrawerSheet
import androidx.compose.material3.ModalNavigationDrawer
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.NavigationDrawerItemDefaults
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.rememberDrawerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.navigation3.runtime.NavKey
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.runtime.rememberNavBackStack
import androidx.navigation3.ui.NavDisplay
import cc.starrytea.horologium.data.Store
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.ui.components.TagChip
import cc.starrytea.horologium.ui.components.TagDot
import cc.starrytea.horologium.ui.nav.HomeRoute
import cc.starrytea.horologium.ui.nav.ProjectsRoute
import cc.starrytea.horologium.ui.nav.RecordRoute
import cc.starrytea.horologium.ui.nav.SettingsRoute
import cc.starrytea.horologium.ui.nav.SummaryRoute
import cc.starrytea.horologium.ui.nav.TagsRoute
import cc.starrytea.horologium.ui.nav.topLevel
import cc.starrytea.horologium.ui.theme.Magnolia
import cc.starrytea.horologium.ui.theme.Radius
import cc.starrytea.horologium.ui.theme.Space
import cc.starrytea.horologium.ui.screen.HomeScreen
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import java.time.Duration
import java.time.Instant
import kotlin.time.Duration.Companion.milliseconds

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AppShell() {
    val backStack = rememberNavBackStack(HomeRoute)
    val drawerState = rememberDrawerState(DrawerValue.Closed)
    val scope = rememberCoroutineScope()

    val active by Store.active.collectAsState()
    LaunchedEffect(Unit) { Store.refresh() }

    val current = backStack.lastOrNull()
    val currentTop = backStack.lastOrNull { key -> topLevel.any { it.route == key } }

    fun goTopLevel(route: NavKey) {
        backStack.clear()
        backStack.add(route)
        scope.launch { drawerState.close() }
    }

    ProvideClock {
        ModalNavigationDrawer(
            drawerState = drawerState,
            scrimColor =
                Magnolia.colors.neutrals.highlight
                    .copy(alpha = 0.32f),
            drawerContent = {
                ModalDrawerSheet(
                    drawerContainerColor = Magnolia.colors.neutrals.cream,
                    drawerContentColor = Magnolia.colors.neutrals.text,
                    drawerShape = RoundedCornerShape(topEnd = Radius.lg, bottomEnd = Radius.lg),
                ) {
                    RunningRow(
                        record = active,
                        onClick = { goTopLevel(HomeRoute) },
                        modifier = Modifier.padding(vertical = Space.s5),
                    )
                    HorizontalDivider(
                        color = Magnolia.colors.border,
                        modifier = Modifier.padding(bottom = Space.s3),
                    )
                    topLevel.forEach { item ->
                        NavigationDrawerItem(
                            label = { Text(item.label, style = Magnolia.type.bodyStrong) },
                            selected = item.route == currentTop,
                            onClick = { goTopLevel(item.route) },
                            shape = Radius.shapeMd,
                            colors =
                                NavigationDrawerItemDefaults.colors(
                                    selectedContainerColor = Magnolia.colors.accent.t25,
                                    unselectedContainerColor = Color.Transparent,
                                    selectedTextColor = Magnolia.colors.neutrals.highlight,
                                    unselectedTextColor = Magnolia.colors.textMuted,
                                    selectedIconColor = Magnolia.colors.neutrals.highlight,
                                    unselectedIconColor = Magnolia.colors.textSubtle,
                                ),
                            modifier = Modifier.padding(horizontal = Space.s3, vertical = Space.s1),
                        )
                    }
                }
            },
        ) {
            Scaffold(
                topBar = {
                    TopAppBar(
                        colors = TopAppBarDefaults.topAppBarColors(containerColor = Magnolia.colors.neutrals.base),
                        title = {},
                        navigationIcon = {
                            IconButton(onClick = { scope.launch { drawerState.open() } }) {
                                Icon(Icons.Default.Menu, contentDescription = "Menu")
                            }
                        },
                        actions = {
                            val running = active
                            if (current != HomeRoute && running != null) {
                                RunningIndicator(record = running, onClick = { goTopLevel(HomeRoute) })
                            }
                        },
                    )
                },
            ) { innerPadding ->
                NavDisplay(
                    backStack = backStack,
                    onBack = { backStack.removeLastOrNull() },
                    modifier = Modifier.padding(innerPadding),
                    entryProvider =
                        entryProvider {
                            entry<HomeRoute> { HomeScreen() }
                            entry<SummaryRoute> { Placeholder("Summary") }
                            entry<ProjectsRoute> { Placeholder("Projects") }
                            entry<TagsRoute> { Placeholder("Tags") }
                            entry<SettingsRoute> { Placeholder("Settings") }
                            entry<RecordRoute> { route -> Placeholder("Record ${route.id}") }
                        },
                )
            }
        }
    }
}

@Composable
private fun Placeholder(name: String) {
    Text(name, style = MaterialTheme.typography.headlineMedium, modifier = Modifier.padding(Space.s4))
}

@Composable
fun RunningRow(
    record: Record?,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val now = rememberNow()

    Row(
        modifier =
            modifier
                .fillMaxWidth()
                .padding(horizontal = Space.s3)
                .clip(Radius.shapeMd)
                .background(if (record != null) Magnolia.colors.neutrals.foam else Color.Transparent)
                .clickable(onClick = onClick)
                .padding(horizontal = Space.s4, vertical = Space.s3),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(
                if (record != null) "Running" else "Nothing running",
                style = Magnolia.type.caption,
                color = Magnolia.colors.textSubtle,
            )
            if (record != null) {
                Text(
                    record.name,
                    style = Magnolia.type.bodyStrong,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
                if (record.tags.isNotEmpty()) {
                    Row(
                        horizontalArrangement = Arrangement.spacedBy(Space.s1),
                        modifier = Modifier.padding(top = Space.s1),
                    ) {
                        record.tags.take(2).forEach { TagChip(it) }
                        if (record.tags.size > 2) {
                            Text("+${record.tags.size - 2}", style = Magnolia.type.caption, color = Magnolia.colors.textSubtle)
                        }
                    }
                }
            }
        }
        if (record != null) {
            Text(
                formatElapsed(Duration.between(record.start, now)),
                style = Magnolia.type.displayLg,
                color = Magnolia.colors.accent.full,
            )
        }
    }
}

@Composable
fun RunningIndicator(
    record: Record,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val now = rememberNow()
    Column(
        horizontalAlignment = Alignment.End,
        modifier =
            modifier
                .clip(Radius.shapeSm)
                .clickable(onClick = onClick)
                .padding(horizontal = Space.s3, vertical = Space.s1),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            record.tags.firstOrNull()?.let {
                TagDot(it)
                Spacer(Modifier.width(Space.s1))
            }
            Text(
                record.name,
                style = Magnolia.type.caption,
                color = Magnolia.colors.textSubtle,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.widthIn(max = 160.dp),
            )
        }
        Text(
            formatElapsed(Duration.between(record.start, now)),
            style = Magnolia.type.bodyStrong,
            color = Magnolia.colors.accent.full,
        )
    }
}

fun formatElapsed(d: Duration): String {
    val s = d.seconds.coerceAtLeast(0)
    return "%d:%02d:%02d".format(s / 3600, (s % 3600) / 60, s % 60)
}

val LocalNow = staticCompositionLocalOf { Instant.now() }

@Composable
fun ProvideClock(content: @Composable () -> Unit) {
    val now by produceState(initialValue = Instant.now()) {
        while (true) {
            val t = Instant.now()
            value = t
            delay((1000 - t.toEpochMilli() % 1000).milliseconds)
        }
    }
    CompositionLocalProvider(LocalNow provides now, content = content)
}

@Composable
fun rememberNow(): Instant = LocalNow.current
