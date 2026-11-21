@file:Suppress("FunctionName")

package cc.starrytea.horologium.ui.components

import androidx.compose.foundation.layout.size
import androidx.compose.material3.Icon
import androidx.compose.material3.LocalContentColor
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.graphics.vector.VectorGroup
import androidx.compose.ui.graphics.vector.VectorPath
import androidx.compose.ui.unit.Dp
import cc.starrytea.horologium.ui.theme.IconSize

fun ImageVector.withStroke(width: Float): ImageVector {
    val builder =
        ImageVector.Builder(
            name = name,
            defaultWidth = defaultWidth,
            defaultHeight = defaultHeight,
            viewportWidth = viewportWidth,
            viewportHeight = viewportHeight,
            tintColor = tintColor,
            tintBlendMode = tintBlendMode,
            autoMirror = autoMirror,
        )
    builder.copyGroup(root, width)
    return builder.build()
}

private fun ImageVector.Builder.copyGroup(
    group: VectorGroup,
    width: Float,
) {
    addGroup(
        name = group.name,
        rotate = group.rotation,
        pivotX = group.pivotX,
        pivotY = group.pivotY,
        scaleX = group.scaleX,
        scaleY = group.scaleY,
        translationX = group.translationX,
        translationY = group.translationY,
        clipPathData = group.clipPathData,
    )
    for (node in group) {
        when (node) {
            is VectorGroup -> {
                copyGroup(node, width)
            }

            is VectorPath -> {
                addPath(
                    pathData = node.pathData,
                    pathFillType = node.pathFillType,
                    name = node.name,
                    fill = node.fill,
                    fillAlpha = node.fillAlpha,
                    stroke = node.stroke,
                    strokeAlpha = node.strokeAlpha,
                    strokeLineWidth = if (node.stroke != null) width else node.strokeLineWidth,
                    strokeLineCap = node.strokeLineCap,
                    strokeLineJoin = node.strokeLineJoin,
                    strokeLineMiter = node.strokeLineMiter,
                    trimPathStart = node.trimPathStart,
                    trimPathEnd = node.trimPathEnd,
                    trimPathOffset = node.trimPathOffset,
                )
            }
        }
    }
    clearGroup()
}

@Composable
fun AppIcon(
    icon: ImageVector,
    contentDescription: String?,
    modifier: Modifier = Modifier,
    size: Dp = IconSize.md,
    stroke: Float = IconSize.stroke,
    tint: Color = LocalContentColor.current,
) {
    val adjusted = remember(icon, stroke) { icon.withStroke(stroke) }
    Icon(
        imageVector = adjusted,
        contentDescription = contentDescription,
        modifier = modifier.size(size),
        tint = tint,
    )
}
