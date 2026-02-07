package com.musicfrog.despicableinfiltrator.ui.connections

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.musicfrog.despicableinfiltrator.ui.common.DEFAULT_FFI_TIMEOUT_MS
import com.musicfrog.despicableinfiltrator.ui.common.LONG_FFI_TIMEOUT_MS
import com.musicfrog.despicableinfiltrator.ui.common.runFfiCall
import com.musicfrog.despicableinfiltrator.ui.common.userMessage
import infiltrator_android.ConnectionRecord
import infiltrator_android.FfiErrorCode
import infiltrator_android.connectionClose
import infiltrator_android.connectionsCloseAll
import infiltrator_android.connectionsList
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

data class ConnectionsUiState(
    val connections: List<ConnectionRecord> = emptyList(),
    val uploadTotal: ULong = 0uL,
    val downloadTotal: ULong = 0uL,
    val isLoading: Boolean = false,
    val error: String? = null,
)

class ConnectionsViewModel : ViewModel() {
    private val allConnections = MutableStateFlow<List<ConnectionRecord>>(emptyList())
    private val uploadTotal = MutableStateFlow(0uL)
    private val downloadTotal = MutableStateFlow(0uL)
    private val isLoading = MutableStateFlow(false)
    private val error = MutableStateFlow<String?>(null)

    private val _hostFilter = MutableStateFlow("")
    private val _processFilter = MutableStateFlow("")

    val hostFilter: StateFlow<String> = _hostFilter
    val processFilter: StateFlow<String> = _processFilter

    val uiState: StateFlow<ConnectionsUiState> = combine(
        allConnections,
        uploadTotal,
        downloadTotal,
        isLoading,
        error,
        _hostFilter,
        _processFilter,
    ) { connections, up, down, loading, err, hostFilter, processFilter ->
        val hostQuery = hostFilter.trim().lowercase()
        val processQuery = processFilter.trim().lowercase()
        val filtered = connections.filter { connection ->
            val hostMatched = if (hostQuery.isBlank()) {
                true
            } else {
                connection.host.lowercase().contains(hostQuery)
            }
            val processMatched = if (processQuery.isBlank()) {
                true
            } else {
                connection.processPath.lowercase().contains(processQuery)
            }
            hostMatched && processMatched
        }
        ConnectionsUiState(
            connections = filtered,
            uploadTotal = up,
            downloadTotal = down,
            isLoading = loading,
            error = err,
        )
    }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5000), ConnectionsUiState())

    init {
        refresh()
    }

    fun setHostFilter(value: String) {
        _hostFilter.value = value
    }

    fun setProcessFilter(value: String) {
        _processFilter.value = value
    }

    fun refresh() {
        viewModelScope.launch {
            loadConnections()
        }
    }

    fun closeConnection(id: String) {
        viewModelScope.launch {
            isLoading.value = true
            error.value = null
            try {
                val call = runFfiCall(timeoutMs = DEFAULT_FFI_TIMEOUT_MS) {
                    connectionClose(id)
                }
                if (call.error != null) {
                    error.value = call.error
                    return@launch
                }
                val status = call.value!!
                if (status.code != FfiErrorCode.OK) {
                    error.value = status.userMessage("Failed to close connection")
                    return@launch
                }
                loadConnections()
            } catch (e: Exception) {
                error.value = e.message
            } finally {
                isLoading.value = false
            }
        }
    }

    fun closeAllConnections() {
        viewModelScope.launch {
            isLoading.value = true
            error.value = null
            try {
                val call = runFfiCall(timeoutMs = LONG_FFI_TIMEOUT_MS) {
                    connectionsCloseAll()
                }
                if (call.error != null) {
                    error.value = call.error
                    return@launch
                }
                val status = call.value!!
                if (status.code != FfiErrorCode.OK) {
                    error.value = status.userMessage("Failed to close all connections")
                    return@launch
                }
                loadConnections()
            } catch (e: Exception) {
                error.value = e.message
            } finally {
                isLoading.value = false
            }
        }
    }

    fun clearError() {
        error.value = null
    }

    private suspend fun loadConnections() {
        isLoading.value = true
        error.value = null
        try {
            val call = runFfiCall(timeoutMs = DEFAULT_FFI_TIMEOUT_MS) {
                connectionsList()
            }
            if (call.error != null) {
                error.value = call.error
                return
            }
            val result = call.value!!
            if (result.status.code == FfiErrorCode.OK) {
                allConnections.value = result.connections
                uploadTotal.value = result.uploadTotal
                downloadTotal.value = result.downloadTotal
            } else {
                error.value = result.status.userMessage("Failed to load connections")
            }
        } catch (e: Exception) {
            error.value = e.message
        } finally {
            isLoading.value = false
        }
    }
}
