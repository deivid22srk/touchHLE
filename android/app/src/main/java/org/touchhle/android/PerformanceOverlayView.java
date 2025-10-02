package org.touchhle.android;

import android.content.Context;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Rect;
import android.os.Debug;
import android.os.Handler;
import android.os.Looper;
import android.util.AttributeSet;
import android.view.View;

public class PerformanceOverlayView extends View {
    private static final long UPDATE_INTERVAL_MS = 100;
    
    private Paint textPaint;
    private Paint backgroundPaint;
    private Handler handler;
    private Runnable updateRunnable;
    
    private boolean showFps = false;
    private boolean showRam = false;
    private int currentFps = 0;
    private float currentRamMB = 0.0f;
    
    private int textSize = 28;
    private int padding = 16;
    
    public PerformanceOverlayView(Context context) {
        super(context);
        init(context);
    }
    
    public PerformanceOverlayView(Context context, AttributeSet attrs) {
        super(context, attrs);
        init(context);
    }
    
    private void init(Context context) {
        textPaint = new Paint(Paint.ANTI_ALIAS_FLAG);
        textPaint.setColor(Color.WHITE);
        textPaint.setTextSize(textSize);
        textPaint.setFakeBoldText(true);
        textPaint.setShadowLayer(4, 2, 2, Color.BLACK);
        
        backgroundPaint = new Paint();
        backgroundPaint.setColor(Color.argb(200, 0, 0, 0));
        
        handler = new Handler(Looper.getMainLooper());
        updateRunnable = new Runnable() {
            @Override
            public void run() {
                updateMetrics();
                if (showFps || showRam) {
                    invalidate();
                    handler.postDelayed(this, UPDATE_INTERVAL_MS);
                }
            }
        };
    }
    
    public void setShowFps(boolean show) {
        this.showFps = show;
        updateVisibility();
    }
    
    public void setShowRam(boolean show) {
        this.showRam = show;
        updateVisibility();
    }
    
    private void updateVisibility() {
        if (showFps || showRam) {
            setVisibility(VISIBLE);
            startUpdating();
        } else {
            setVisibility(GONE);
            stopUpdating();
        }
    }
    
    private void startUpdating() {
        handler.removeCallbacks(updateRunnable);
        handler.post(updateRunnable);
    }
    
    private void stopUpdating() {
        handler.removeCallbacks(updateRunnable);
    }
    
    private void updateMetrics() {
        if (showFps) {
            try {
                currentFps = TouchHLENative.getCurrentFps();
            } catch (Exception e) {
                currentFps = 0;
            }
        }
        
        if (showRam) {
            calculateRam();
        }
    }
    
    private void calculateRam() {
        Debug.MemoryInfo memoryInfo = new Debug.MemoryInfo();
        Debug.getMemoryInfo(memoryInfo);
        
        int totalPss = memoryInfo.getTotalPss();
        currentRamMB = totalPss / 1024.0f;
        
        if (currentRamMB < 1.0f) {
            Runtime runtime = Runtime.getRuntime();
            long appMemory = (runtime.totalMemory() - runtime.freeMemory());
            currentRamMB = appMemory / (1024.0f * 1024.0f);
        }
    }
    
    @Override
    protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);
        
        if (!showFps && !showRam) {
            return;
        }
        
        int yOffset = padding + 20;
        
        if (showFps) {
            String fpsText = String.format("FPS: %d", currentFps);
            Rect bounds = new Rect();
            textPaint.getTextBounds(fpsText, 0, fpsText.length(), bounds);
            
            int boxWidth = bounds.width() + padding * 3;
            int boxHeight = bounds.height() + padding * 2;
            
            canvas.drawRoundRect(
                padding,
                yOffset,
                padding + boxWidth,
                yOffset + boxHeight,
                12, 12,
                backgroundPaint
            );
            
            canvas.drawText(
                fpsText,
                padding * 2,
                yOffset + bounds.height() + padding,
                textPaint
            );
            
            yOffset += boxHeight + padding / 2;
        }
        
        if (showRam) {
            String ramText = String.format("RAM: %.0f MB", currentRamMB);
            Rect bounds = new Rect();
            textPaint.getTextBounds(ramText, 0, ramText.length(), bounds);
            
            int boxWidth = bounds.width() + padding * 3;
            int boxHeight = bounds.height() + padding * 2;
            
            canvas.drawRoundRect(
                padding,
                yOffset,
                padding + boxWidth,
                yOffset + boxHeight,
                12, 12,
                backgroundPaint
            );
            
            canvas.drawText(
                ramText,
                padding * 2,
                yOffset + bounds.height() + padding,
                textPaint
            );
        }
    }
    
    @Override
    protected void onDetachedFromWindow() {
        super.onDetachedFromWindow();
        stopUpdating();
    }
}
