package cc.starrytea.horologium.ui.theme

import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
import androidx.compose.material3.Typography
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.ReadOnlyComposable
import androidx.compose.runtime.remember
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.googlefonts.Font
import androidx.compose.ui.text.googlefonts.GoogleFont
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp
import cc.starrytea.horologium.R
import kotlin.math.PI
import kotlin.math.atan2
import kotlin.math.cbrt
import kotlin.math.cos
import kotlin.math.pow
import kotlin.math.sin
import kotlin.math.sqrt

object Hues {
    val Strawberry = Color(0xFFCF8282)
    val Orange = Color(0xFFD4A266)
    val Lemon = Color(0xFFE3CD7F)
    val Leaf = Color(0xFF9CCC96)
    val Mint = Color(0xFF99CFAC)
    val Sky = Color(0xFF90CDDE)
    val Blueberry = Color(0xFF9BB0DE)
    val Grape = Color(0xFF9B98D6)
    val Plum = Color(0xFFAA99D1)
    val Lavender = Color(0xFFCCA3D6)
    val Lilac = Color(0xFFDBB4D3)
    val Pink = Color(0xFFEDB7CA)
}

@Immutable
data class Neutrals(
    val foam: Color,
    val cream: Color,
    val base: Color,
    val tea: Color,
    val subtext2: Color,
    val subtext1: Color,
    val text: Color,
    val highlight: Color,
)

val MilkTea =
    Neutrals(
        foam = Color(0xFFF8F4E9),
        cream = Color(0xFFF0E8DA),
        base = Color(0xFFE6D9C8),
        tea = Color(0xFFDECEC1),
        subtext2 = Color(0xFF6B564B),
        subtext1 = Color(0xFF4F453F),
        text = Color(0xFF403732),
        highlight = Color(0xFF383029),
    )

val Tiramisu =
    Neutrals(
        foam = Color(0xFF6B564B),
        cream = Color(0xFF4F453F),
        base = Color(0xFF403732),
        tea = Color(0xFF383029),
        subtext2 = Color(0xFFDECEC1),
        subtext1 = Color(0xFFE6D9C8),
        text = Color(0xFFF0E8DA),
        highlight = Color(0xFFF8F4E9),
    )

object Mix {
    const val S1 = 0.04f
    const val S2 = 0.08f
    const val S3 = 0.15f
    const val S4 = 0.25f
    const val S5 = 0.40f
    const val S6 = 0.60f
}

@Immutable
data class Tints(
    val full: Color,
    val t4: Color,
    val t8: Color,
    val t15: Color,
    val t25: Color,
    val t40: Color,
    val t60: Color,
) {
    companion object {
        fun of(
            color: Color,
            base: Color,
        ) = Tints(
            full = color,
            t4 = mixOklch(color, Mix.S1, base),
            t8 = mixOklch(color, Mix.S2, base),
            t15 = mixOklch(color, Mix.S3, base),
            t25 = mixOklch(color, Mix.S4, base),
            t40 = mixOklch(color, Mix.S5, base),
            t60 = mixOklch(color, Mix.S6, base),
        )
    }
}

@Immutable
data class MagnoliaColors(
    val neutrals: Neutrals,
    val accent: Tints,
    val success: Tints,
    val warning: Tints,
    val error: Tints,
    val isDark: Boolean,
) {
    val border
        get() = neutrals.tea
    val textMuted
        get() = neutrals.subtext1
    val textSubtle
        get() = neutrals.subtext2
    val floatShadow
        get() = neutrals.subtext2.copy(alpha = 0.15f)
}

fun magnoliaColors(
    dark: Boolean,
    accent: Color = if (dark) Hues.Pink else Hues.Strawberry,
    success: Color = Hues.Leaf,
    warning: Color = Hues.Orange,
    error: Color = Hues.Strawberry,
): MagnoliaColors {
    val n = if (dark) Tiramisu else MilkTea
    return MagnoliaColors(
        neutrals = n,
        accent = Tints.of(accent, n.base),
        success = Tints.of(success, n.base),
        warning = Tints.of(warning, n.base),
        error = Tints.of(error, n.base),
        isDark = dark,
    )
}

fun mixOklch(
    a: Color,
    amountA: Float,
    b: Color,
): Color {
    val la = a.toOklch()
    val lb = b.toOklch()
    val t = 1f - amountA
    val l = la[0] + (lb[0] - la[0]) * t
    val c = la[1] + (lb[1] - la[1]) * t
    val ha = if (la[1] < 1e-4f) lb[2] else la[2]
    val hb = if (lb[1] < 1e-4f) la[2] else lb[2]
    var dh = hb - ha
    if (dh > 180f) dh -= 360f
    if (dh < -180f) dh += 360f
    val h = ha + dh * t
    return oklchToColor(l, c, h, a.alpha + (b.alpha - a.alpha) * t)
}

private fun toLinear(c: Float): Float = if (c <= 0.04045f) c / 12.92f else ((c + 0.055f) / 1.055f).pow(2.4f)

private fun toGamma(c: Float): Float = if (c <= 0.0031308f) c * 12.92f else 1.055f * c.pow(1f / 2.4f) - 0.055f

private fun Color.toOklch(): FloatArray {
    val r = toLinear(red)
    val g = toLinear(green)
    val b = toLinear(blue)
    val l = cbrt(0.4122214708f * r + 0.5363325363f * g + 0.0514459929f * b)
    val m = cbrt(0.2119034982f * r + 0.6806995451f * g + 0.1073969566f * b)
    val s = cbrt(0.0883024619f * r + 0.2817188376f * g + 0.6299787005f * b)
    val L = 0.2104542553f * l + 0.7936177850f * m - 0.0040720468f * s
    val A = 1.9779984951f * l - 2.4285922050f * m + 0.4505937099f * s
    val B = 0.0259040371f * l + 0.7827717662f * m - 0.8086757660f * s
    val C = sqrt(A * A + B * B)
    var H = (atan2(B, A) * 180f / PI.toFloat())
    if (H < 0f) H += 360f
    return floatArrayOf(L, C, H)
}

private fun oklchToColor(
    L: Float,
    C: Float,
    H: Float,
    alpha: Float,
): Color {
    val hr = H * PI.toFloat() / 180f
    val A = C * cos(hr)
    val B = C * sin(hr)
    val l = (L + 0.3963377774f * A + 0.2158037573f * B).pow(3)
    val m = (L - 0.1055613458f * A - 0.0638541728f * B).pow(3)
    val s = (L - 0.0894841775f * A - 1.2914855480f * B).pow(3)
    val r = 4.0767416621f * l - 3.3077115913f * m + 0.2309699292f * s
    val g = -1.2684380046f * l + 2.6097574011f * m - 0.3413193965f * s
    val b = -0.0041960863f * l - 0.7034186147f * m + 1.7076147010f * s
    return Color(
        toGamma(r).coerceIn(0f, 1f),
        toGamma(g).coerceIn(0f, 1f),
        toGamma(b).coerceIn(0f, 1f),
        alpha.coerceIn(0f, 1f),
    )
}

private val GoogleFontsProvider =
    GoogleFont.Provider(
        providerAuthority = "com.google.android.gms.fonts",
        providerPackage = "com.google.android.gms",
        certificates = R.array.com_google_android_gms_fonts_certs,
    )

private fun googleFamily(
    name: String,
    vararg weights: FontWeight,
): FontFamily {
    val font = GoogleFont(name)
    return FontFamily(
        weights.map { Font(googleFont = font, fontProvider = GoogleFontsProvider, weight = it) },
    )
}

object MagnoliaFonts {
    val Display =
        FontFamily(
            Font(R.font.fredoka_regular, FontWeight.Normal),
            Font(R.font.fredoka_medium, FontWeight.Medium),
            Font(R.font.fredoka_semibold, FontWeight.SemiBold),
        )
    val Body = googleFamily("Nunito", FontWeight.Normal, FontWeight.Medium, FontWeight.SemiBold)
    val Mono =
        FontFamily(
            Font(R.font.monaspace_neon_regular, FontWeight.Normal),
        )
}

object TextSize {
    val xs = 12.sp
    val sm = 14.sp
    val base = 16.sp
    val lg = 20.sp
    val xl = 25.sp
    val x2l = 31.25.sp
    val x3l = 39.06.sp
}

object Leading {
    val heading = 1.2.em
    val body = 1.6.em
    val small = 1.5.em
}

@Immutable
data class MagnoliaType(
    val display3xl: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Display,
            fontWeight = FontWeight.SemiBold,
            fontSize = TextSize.x3l,
            lineHeight = Leading.heading,
        ),
    val display2xl: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Display,
            fontWeight = FontWeight.SemiBold,
            fontSize = TextSize.x2l,
            lineHeight = Leading.heading,
        ),
    val displayXl: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Display,
            fontWeight = FontWeight.Medium,
            fontSize = TextSize.xl,
            lineHeight = Leading.heading,
        ),
    val displayLg: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Display,
            fontWeight = FontWeight.Medium,
            fontSize = TextSize.lg,
            lineHeight = Leading.heading,
        ),
    val body: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Body,
            fontWeight = FontWeight.Normal,
            fontSize = TextSize.base,
            lineHeight = Leading.body,
        ),
    val bodyStrong: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Body,
            fontWeight = FontWeight.SemiBold,
            fontSize = TextSize.base,
            lineHeight = Leading.body,
        ),
    val small: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Body,
            fontWeight = FontWeight.Normal,
            fontSize = TextSize.sm,
            lineHeight = Leading.small,
        ),
    val caption: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Body,
            fontWeight = FontWeight.Medium,
            fontSize = TextSize.xs,
            lineHeight = Leading.small,
        ),
    val mono: TextStyle =
        TextStyle(
            fontFamily = MagnoliaFonts.Mono,
            fontWeight = FontWeight.Normal,
            fontSize = TextSize.sm,
            lineHeight = Leading.small,
        ),
)

object Space {
    val s1 = 4.dp
    val s2 = 8.dp
    val s3 = 12.dp
    val s4 = 16.dp
    val s5 = 24.dp
    val s6 = 32.dp
    val s7 = 48.dp
    val s8 = 64.dp
}

object Radius {
    val sm = 8.dp
    val md = 16.dp
    val lg = 24.dp
    val shapeSm: Shape = RoundedCornerShape(sm)
    val shapeMd: Shape = RoundedCornerShape(md)
    val shapeLg: Shape = RoundedCornerShape(lg)
    val shapeFull: Shape = RoundedCornerShape(percent = 50)
}

object Borders {
    val thin = 1.dp
    val thick = 2.dp
}

object Motion {
    const val fast = 150
    const val base = 250
    val ease = CubicBezierEasing(0.2f, 0f, 0f, 1f)
}

object IconSize {
    val sm = 16.dp
    val md = 20.dp
    const val stroke = 1.5f
}

val MaxMeasure: Dp = 640.dp

private val LocalMagnoliaColors = staticCompositionLocalOf { magnoliaColors(dark = false) }
private val LocalMagnoliaType = staticCompositionLocalOf { MagnoliaType() }

object Magnolia {
    val colors: MagnoliaColors
        @Composable @ReadOnlyComposable
        get() = LocalMagnoliaColors.current
    val type: MagnoliaType
        @Composable @ReadOnlyComposable
        get() = LocalMagnoliaType.current
}

fun Modifier.floatShadow(
    colors: MagnoliaColors,
    shape: Shape = Radius.shapeLg,
) = shadow(
    elevation = 12.dp,
    shape = shape,
    ambientColor = colors.floatShadow,
    spotColor = colors.floatShadow,
)

@Composable fun thinBorder() = BorderStroke(Borders.thin, Magnolia.colors.border)

@Composable
fun MagnoliaTheme(
    dark: Boolean = isSystemInDarkTheme(),
    accent: Color? = null,
    content: @Composable () -> Unit,
) {
    val colors =
        remember(dark, accent) {
            if (accent != null) magnoliaColors(dark, accent = accent) else magnoliaColors(dark)
        }
    val type = remember { MagnoliaType() }
    val n = colors.neutrals

    val scheme =
        if (dark) {
            darkColorScheme(
                primary = colors.accent.full,
                onPrimary = n.base,
                primaryContainer = colors.accent.t25,
                onPrimaryContainer = n.highlight,
                background = n.base,
                onBackground = n.text,
                surface = n.cream,
                onSurface = n.text,
                surfaceVariant = n.foam,
                onSurfaceVariant = n.subtext1,
                outline = n.tea,
                outlineVariant = n.tea,
                error = colors.error.full,
                errorContainer = colors.error.t25,
            )
        } else {
            lightColorScheme(
                primary = colors.accent.full,
                onPrimary = n.highlight,
                primaryContainer = colors.accent.t25,
                onPrimaryContainer = n.highlight,
                background = n.base,
                onBackground = n.text,
                surface = n.cream,
                onSurface = n.text,
                surfaceVariant = n.foam,
                onSurfaceVariant = n.subtext1,
                outline = n.tea,
                outlineVariant = n.tea,
                error = colors.error.full,
                errorContainer = colors.error.t25,
            )
        }

    val m3Type =
        Typography(
            displayLarge = type.display3xl,
            displayMedium = type.display2xl,
            headlineLarge = type.displayXl,
            headlineMedium = type.displayLg,
            titleLarge = type.displayLg,
            titleMedium = type.bodyStrong,
            bodyLarge = type.body,
            bodyMedium = type.small,
            bodySmall = type.caption,
            labelLarge = type.bodyStrong.copy(fontSize = TextSize.sm),
            labelMedium = type.caption,
            labelSmall = type.caption,
        )

    val shapes =
        Shapes(
            small = RoundedCornerShape(Radius.sm),
            medium = RoundedCornerShape(Radius.md),
            large = RoundedCornerShape(Radius.lg),
        )

    CompositionLocalProvider(
        LocalMagnoliaColors provides colors,
        LocalMagnoliaType provides type,
    ) { MaterialTheme(colorScheme = scheme, typography = m3Type, shapes = shapes, content = content) }
}
