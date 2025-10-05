package org.touchhle.android

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.Rect
import android.os.Debug
import android.os.Handler
import android.os.Looper
import android.util.AttributeSet
import android.view.View

class PerformanceOverlayView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : View(context, attrs) {

    companion object {
        private const val UPDATE_INTERVAL_MS = 100L
    }

    private val textPaint: Paint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        color = Color.WHITE
        textSize = 28f
        isFakeBoldText = true
        setShadowLayer(4f, 2f, 2f, Color.BLACK)
    }

    private val backgroundPaint: Paint = Paint().apply {
        color = Color.argb(200, 0, 0, 0)
    }

    private val handler = Handler(Looper.getMainLooper())
    private val updateRunnable = object : Runnable {
        override fun run() {
            updateMetrics()
            if (showFps || showRam) {
                invalidate()
                handler.postDelayed(this, UPDATE_INTERVAL_MS)
            }
        }
    }

    var showFps: Boolean = false
        set(value) {
            field = value
            updateVisibility()
        }

    var showRam: Boolean = false
        set(value) {
            field = value
            updateVisibility()
        }

    private var currentFps: Int = 0
    private var currentRamMB: Float = 0.0f

    private val textSize = 28
    private val padding = 16

    private fun updateVisibility() {
        if (showFps || showRam) {
            visibility = VISIBLE
            startUpdating()
        } else {
            visibility = GONE
            stopUpdating()
        }
    }

    private fun startUpdating() {
        handler.removeCallbacks(updateRunnable)
        handler.post(updateRunnable)
    }

    private fun stopUpdating() {
        handler.removeCallbacks(updateRunnable)
    }

    private fun updateMetrics() {
        if (showFps) {
            try {
                currentFps = TouchHLENative.getCurrentFps()
            } catch (e: Exception) {
                currentFps = 0
            }
        }

        if (showRam) {
            calculateRam()
        }
    }

    private fun calculateRam() {
        val memoryInfo = Debug.MemoryInfo()
        Debug.getMemoryInfo(memoryInfo)

        val totalPss = memoryInfo.totalPss
        currentRamMB = totalPss / 1024.0f

        if (currentRamMB < 1.0f) {
            val runtime = Runtime.getRuntime()
            val appMemory = runtime.totalMemory() - runtime.freeMemory()
            currentRamMB = appMemory / (1024.0f * 1024.0f)
        }
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        if (!showFps && !showRam) {
            return
        }

        var yOffset = padding + 20

        if (showFps) {
            val fpsText = String.format("FPS: %d", currentFps)
            val bounds = Rect()
            textPaint.getTextBounds(fpsText, 0, fpsText.length, bounds)

            val boxWidth = bounds.width() + padding * 3
            val boxHeight = bounds.height() + padding * 2

            canvas.drawRoundRect(
                padding.toFloat(),
                yOffset.toFloat(),
                (padding + boxWidth).toFloat(),
                (yOffset + boxHeight).toFloat(),
                12f, 12f,
                backgroundPaint
            )

            canvas.drawText(
                fpsText,
                (padding * 2).toFloat(),
                (yOffset + bounds.height() + padding).toFloat(),
                textPaint
            )

            yOffset += boxHeight + padding / 2
        }

        if (showRam) {
            val ramText = String.format("RAM: %.0f MB", currentRamMB)
            val bounds = Rect()
            textPaint.getTextBounds(ramText, 0, ramText.length, bounds)

            val boxWidth = bounds.width() + padding * 3
            val boxHeight = bounds.height() + padding * 2

            canvas.drawRoundRect(
                padding.toFloat(),
                yOffset.toFloat(),
                (padding + boxWidth).toFloat(),
                (yOffset + boxHeight).toFloat(),
                12f, 12f,
                backgroundPaint
            )

            canvas.drawText(
                ramText,
                (padding * 2).toFloat(),
                (yOffset + bounds.height() + padding).toFloat(),
                textPaint
            )
        }
    }

    override fun onDetachedFromWindow() {
        super.onDetachedFromWindow()
        stopUpdating()
    }
}
