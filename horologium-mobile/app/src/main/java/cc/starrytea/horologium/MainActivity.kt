package cc.starrytea.horologium

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import cc.starrytea.horologium.data.FakeRecordStore
import cc.starrytea.horologium.data.RecordStore
import cc.starrytea.horologium.ui.theme.*

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            val store = remember { FakeRecordStore() }
            MagnoliaTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Home(store = store, modifier = Modifier.padding(innerPadding))
                }
            }
        }
    }
}

@Composable
fun Home(
    store: RecordStore,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier.fillMaxSize().padding(Space.s4)) {
        val active = store.active()
        Text(active?.name ?: "Nothing running", style = MaterialTheme.typography.headlineMedium)
        Button(onClick = { if (active != null) store.stop(active.id) else store.start("Test entry") }) {
            Text(if (active != null) "Stop" else "Start")
        }
        LazyColumn {
            items(store.records.reversed()) { record ->
                Text(record.name, style = MaterialTheme.typography.bodyLarge)
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun HomePreview(store: RecordStore) {
    MagnoliaTheme { Home(store = store) }
}
