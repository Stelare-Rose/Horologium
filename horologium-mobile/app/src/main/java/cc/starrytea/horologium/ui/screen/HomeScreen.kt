@file:Suppress("FunctionName")

package cc.starrytea.horologium.ui.screen

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextOverflow
import cc.starrytea.horologium.data.Store
import cc.starrytea.horologium.model.Project
import cc.starrytea.horologium.model.Record
import cc.starrytea.horologium.ui.components.AppIcon
import cc.starrytea.horologium.ui.components.Chip
import cc.starrytea.horologium.ui.components.ChipSize
import cc.starrytea.horologium.ui.components.TagChip
import cc.starrytea.horologium.ui.formatElapsed
import cc.starrytea.horologium.ui.rememberNow
import cc.starrytea.horologium.ui.theme.Borders
import cc.starrytea.horologium.ui.theme.IconSize
import cc.starrytea.horologium.ui.theme.Magnolia
import cc.starrytea.horologium.ui.theme.MaxMeasure
import cc.starrytea.horologium.ui.theme.Radius
import cc.starrytea.horologium.ui.theme.Space
import compose.icons.TablerIcons
import compose.icons.tablericons.Folder
import compose.icons.tablericons.Tag
import java.time.Duration

@Composable
fun HomeScreen(modifier: Modifier = Modifier) {
    val active by Store.active.collectAsState()

    Column(
        modifier =
            modifier
                .fillMaxSize()
                .padding(Space.s4),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        RunningCard(
            record = active,
            modifier =
                Modifier
                    .widthIn(max = MaxMeasure)
                    .fillMaxWidth(),
        )
    }
}

@Composable
fun RunningCard(
    record: Record?,
    modifier: Modifier = Modifier,
) {
    Card(
        modifier = modifier,
        shape = Radius.shapeLg,
        colors =
            CardDefaults.cardColors(
                containerColor = Magnolia.colors.neutrals.foam,
                contentColor = Magnolia.colors.neutrals.text,
            ),
        border = BorderStroke(Borders.thin, Magnolia.colors.border),
    ) {
        Column(
            modifier = Modifier.padding(Space.s5),
            verticalArrangement = Arrangement.spacedBy(Space.s3),
        ) {
            if (record == null) {
                Text(
                    "Nothing running",
                    style = Magnolia.type.displayLg,
                    color = Magnolia.colors.textSubtle,
                )
            } else {
                RunningHeader(record)
                TagsSection(record)
                ProjectSection(record.project)
            }
        }
    }
}

@Composable
private fun RunningHeader(record: Record) {
    val now = rememberNow()
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(Space.s4),
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(
                "Currently Running",
                style = Magnolia.type.caption,
                color = Magnolia.colors.textSubtle,
            )
            Text(
                record.name,
                style = Magnolia.type.displayXl,
                color = Magnolia.colors.neutrals.highlight,
                maxLines = 2,
                overflow = TextOverflow.Ellipsis,
            )
        }
        Text(
            formatElapsed(Duration.between(record.start, now)),
            style = Magnolia.type.displayLg,
            color = Magnolia.colors.accent.full,
        )
    }
}

@Composable
private fun TagsSection(record: Record) {
    if (record.tags.isNotEmpty()) {
        FlowRow(
            horizontalArrangement = Arrangement.spacedBy(Space.s2),
            verticalArrangement = Arrangement.spacedBy(Space.s2),
        ) {
            AppIcon(
                icon = TablerIcons.Tag,
                modifier = Modifier.size(IconSize.md).align(Alignment.CenterVertically),
                contentDescription = "Tags",
            )
            record.tags.forEach { TagChip(it, size = ChipSize.Md) }
        }
    }
}

@Composable
private fun ProjectSection(project: Project?) {
    if (project != null) {
        Row(horizontalArrangement = Arrangement.spacedBy(Space.s2)) {
            AppIcon(
                icon = TablerIcons.Folder,
                modifier = Modifier.size(IconSize.md).align(Alignment.CenterVertically),
                contentDescription = "Folder",
            )
            Chip(color = project.color(Magnolia.colors.accent.full)) {
                Text(project.name, maxLines = 1)
            }
        }
    }
}

@Composable
private fun Section(
    title: String,
    content: @Composable () -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(Space.s2)) {
        Text(
            title,
            style = Magnolia.type.caption,
            color = Magnolia.colors.textSubtle,
        )
        content()
    }
}

@Composable
private fun EmptyLine(text: String) {
    Text(text, style = Magnolia.type.small, color = Magnolia.colors.textSubtle)
}

fun Project.color(fallback: Color): Color =
    colors
        .firstOrNull()
        ?.removePrefix("#")
        ?.toLongOrNull(16)
        ?.let { Color(0xFF000000 or it) }
        ?: fallback
