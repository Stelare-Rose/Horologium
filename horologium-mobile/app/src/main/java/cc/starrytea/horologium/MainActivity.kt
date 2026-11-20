package cc.starrytea.horologium

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.os.Environment
import android.provider.Settings
import android.util.Log
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
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import cc.starrytea.horologium.Native
import cc.starrytea.horologium.data.RecordStore
import cc.starrytea.horologium.data.Store
import cc.starrytea.horologium.ui.AppShell
import cc.starrytea.horologium.ui.screen.PermissionScreen
import cc.starrytea.horologium.ui.theme.*
import cc.starrytea.horologium.wrapper.Horologium
import cc.starrytea.horologium.wrapper.loadOrCreateDeviceId

class MainActivity : ComponentActivity() {
    private var hasAccess by mutableStateOf(false)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MagnoliaTheme {
                if (hasAccess) AppShell() else PermissionScreen(onGrant = { openAllFilesSettings() })
            }
        }
    }

    override fun onResume() {
        super.onResume()
        if (Environment.isExternalStorageManager()) {
            if (!hasAccess) {
                hasAccess = true
                init()
            }
        } else {
            hasAccess = false
        }
    }

    private fun openAllFilesSettings() {
        startActivity(
            Intent(
                Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION,
                Uri.parse("package:$packageName"),
            ),
        )
    }

    private fun init() {
        Horologium.init(this, loadOrCreateDeviceId(this))
        Store.init(this)
    }
}

@Composable
fun HomeScreen() {
    MagnoliaTheme {
        Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
        }
    }
}
