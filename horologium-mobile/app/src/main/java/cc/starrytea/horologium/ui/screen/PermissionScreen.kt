package cc.starrytea.horologium.ui.screen

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import cc.starrytea.horologium.ui.theme.Space

@Composable
fun PermissionScreen(onGrant: () -> Unit) {
    Scaffold {
        Column(
            modifier = Modifier.fillMaxSize().padding(Space.s3),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(
                "Horologium needs permission to save records in a public folder.",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
            )
            Text(
                "After granting access, you can sync the /Horologium folder with your preferred sync tool.",
                style = MaterialTheme.typography.bodyMedium,
                textAlign = TextAlign.Center,
            )
            Button(
                modifier = Modifier.padding(Space.s3),
                onClick = onGrant,
            ) {
                Text("Grant access")
            }
        }
    }
}
