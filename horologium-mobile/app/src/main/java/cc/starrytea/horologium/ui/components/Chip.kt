@file:Suppress("FunctionName")

package cc.starrytea.horologium.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.focus.focusProperties
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import cc.starrytea.horologium.model.Tag
import cc.starrytea.horologium.ui.theme.Borders
import cc.starrytea.horologium.ui.theme.IconSize
import cc.starrytea.horologium.ui.theme.Magnolia
import cc.starrytea.horologium.ui.theme.MagnoliaFonts
import cc.starrytea.horologium.ui.theme.Mix
import cc.starrytea.horologium.ui.theme.Radius
import cc.starrytea.horologium.ui.theme.Space
import cc.starrytea.horologium.ui.theme.TextSize

enum class ChipSize { Sm, Md }

@Composable
fun Chip(
    modifier: Modifier = Modifier,
    color: Color? = null,
    size: ChipSize = ChipSize.Md,
    onRemove: (() -> Unit)? = null,
    removeLabel: String = "Remove",
    focusable: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    val chipColor = color ?: Magnolia.colors.accent.full
    val removable = onRemove != null

    val fontSize = if (size == ChipSize.Md) TextSize.sm else TextSize.xs
    val style =
        TextStyle(
            fontFamily = MagnoliaFonts.Display,
            fontWeight = FontWeight.Medium,
            fontSize = fontSize,
            lineHeight = 1.4.em,
            color = chipColor,
        )

    val padding =
        when {
            size == ChipSize.Md && removable -> PaddingValues(start = 10.5.dp, end = Space.s1, top = 3.5.dp, bottom = 3.5.dp)
            size == ChipSize.Md -> PaddingValues(horizontal = 10.5.dp, vertical = 3.5.dp)
            removable -> PaddingValues(start = Space.s2)
            else -> PaddingValues(horizontal = Space.s2)
        }

    CompositionLocalProvider(LocalContentColor provides chipColor) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = if (removable) Arrangement.spacedBy(Space.s2) else Arrangement.Start,
            modifier =
                modifier
                    .clip(Radius.shapeFull)
                    .background(chipColor.copy(alpha = Mix.S3))
                    .dashedBorder(chipColor)
                    .padding(padding),
        ) {
            androidx.compose.material3.ProvideTextStyle(style) {
                content()
            }
            if (onRemove != null) {
                RemoveButton(chipColor, onRemove, removeLabel, focusable)
            }
        }
    }
}

@Composable
private fun RemoveButton(
    chipColor: Color,
    onRemove: () -> Unit,
    label: String,
    focusable: Boolean,
) {
    val interaction = remember { MutableInteractionSource() }
    val hovered by interaction.collectIsHoveredAsState()
    val pressed by interaction.collectIsPressedAsState()
    val focused by interaction.collectIsFocusedAsState()

    Box(
        contentAlignment = Alignment.Center,
        modifier =
            Modifier
                .size(IconSize.sm)
                .focusProperties { canFocus = focusable }
                .then(
                    if (focused) {
                        Modifier.drawBehind {
                            val inset = -1.dp.toPx() - Borders.thick.toPx() / 2
                            drawRoundRect(
                                color = chipColor,
                                topLeft = Offset(inset, inset),
                                size = Size(size.width - inset * 2, size.height - inset * 2),
                                cornerRadius = CornerRadius(size.height),
                                style = Stroke(width = Borders.thick.toPx()),
                            )
                        }
                    } else {
                        Modifier
                    },
                ).clip(Radius.shapeFull)
                .background(if (hovered || pressed) chipColor.copy(alpha = Mix.S4) else Color.Transparent)
                .clickable(
                    interactionSource = interaction,
                    indication = null,
                    role = Role.Button,
                    onClick = onRemove,
                ).semantics { contentDescription = label },
    ) {
        Text("✕", fontSize = TextSize.xs, color = chipColor)
    }
}

private fun Modifier.dashedBorder(color: Color) =
    drawBehind {
        val stroke = Borders.thin.toPx()
        val dash = 3.dp.toPx()
        drawRoundRect(
            color = color,
            topLeft = Offset(stroke / 2, stroke / 2),
            size = Size(size.width - stroke, size.height - stroke),
            cornerRadius = CornerRadius(size.height / 2),
            style =
                Stroke(
                    width = stroke,
                    pathEffect = PathEffect.dashPathEffect(floatArrayOf(dash, dash)),
                ),
        )
    }

fun Tag.color(fallback: Color): Color =
    colors
        .firstOrNull()
        ?.removePrefix("#")
        ?.toLongOrNull(16)
        ?.let { Color(0xFF000000 or it) }
        ?: fallback

@Composable
fun TagChip(
    tag: Tag,
    modifier: Modifier = Modifier,
    size: ChipSize = ChipSize.Sm,
    onRemove: (() -> Unit)? = null,
) {
    Chip(
        modifier = modifier,
        color = tag.color(Magnolia.colors.accent.full),
        size = size,
        onRemove = onRemove,
        removeLabel = "Remove ${tag.name}",
    ) {
        Text(tag.name, maxLines = 1)
    }
}

@Composable
fun TagDot(
    tag: Tag,
    modifier: Modifier = Modifier,
) {
    Box(
        modifier
            .size(8.dp)
            .background(tag.color(Magnolia.colors.accent.full), Radius.shapeFull),
    )
}
