/**
 * NeuroSync rPPG Engine (Remote Photoplethysmography Core)
 * Processa a micro-variação de absorção do canal verde facial e calcula
 * métricas de resposta autônoma cardíaca em tempo real.
 */
class NeuroSyncRPPG {
    constructor(sampleRateFps = 30, bufferSize = 150) {
        this.fps = sampleRateFps;
        this.bufferSize = bufferSize; // Janela de 5 segundos a 30 FPS
        this.greenSignalBuffer = [];
        this.timestamps = [];
        this.previousBpm = 70.0;
        this.lastBpmTime = performance.now();
    }

    /**
     * Extrai a média de intensidade do Canal Verde (Green Channel) na ROI (Testa)
     */
    extractROIIntensity(ctx, landmarks, canvasWidth, canvasHeight) {
        if (!landmarks || landmarks.length < 300) return null;

        // Ponto 10 do MediaPipe é o centro da testa
        const foreheadX = Math.floor(landmarks[10].x * canvasWidth);
        const foreheadY = Math.floor(landmarks[10].y * canvasHeight);
        const roiWidth = 30;
        const roiHeight = 15;

        const startX = Math.max(0, foreheadX - 15);
        const startY = Math.max(0, foreheadY - 7);

        try {
            const imgData = ctx.getImageData(startX, startY, roiWidth, roiHeight);
            const data = imgData.data;
            let greenSum = 0;
            let count = 0;

            for (let i = 0; i < data.length; i += 4) {
                greenSum += data[i + 1]; // Offset +1 = Canal Verde
                count++;
            }

            const meanGreen = count > 0 ? greenSum / count : 0;
            this.pushSignal(meanGreen);
            return meanGreen;
        } catch (e) {
            return null;
        }
    }

    pushSignal(val) {
        const now = performance.now();
        this.greenSignalBuffer.push(val);
        this.timestamps.push(now);

        if (this.greenSignalBuffer.length > this.bufferSize) {
            this.greenSignalBuffer.shift();
            this.timestamps.shift();
        }
    }

    /**
     * Processa a janela temporal e calcula Batimentos (BPM), Aceleração, Desaceleração e HRV
     */
    computeHeartRateMetrics() {
        if (this.greenSignalBuffer.length < this.bufferSize) {
            return {
                bpm: 72.0,
                acceleration_bpm_s: 0.0,
                deceleration_bpm_s: 0.0,
                hrv_rmssd_ms: 42.0
            };
        }

        // 1. Normalização do Sinal
        const mean = this.greenSignalBuffer.reduce((a, b) => a + b, 0) / this.greenSignalBuffer.length;
        const normalized = this.greenSignalBuffer.map(v => v - mean);

        // 2. Detecção Simplificada de Picos de Pulso Arterial
        let peaks = [];
        for (let i = 1; i < normalized.length - 1; i++) {
            if (normalized[i] > normalized[i - 1] && normalized[i] > normalized[i + 1] && normalized[i] > 0.5) {
                peaks.push(this.timestamps[i]);
            }
        }

        let currentBpm = this.previousBpm;
        let hrvRmssd = 45.0;

        if (peaks.length >= 2) {
            let intervals = [];
            for (let j = 1; j < peaks.length; j++) {
                intervals.push(peaks[j] - peaks[j - 1]);
            }

            const avgIntervalMs = intervals.reduce((a, b) => a + b, 0) / intervals.length;
            if (avgIntervalMs > 300 && avgIntervalMs < 1300) {
                currentBpm = parseFloat((60000 / avgIntervalMs).toFixed(1));
            }

            // Cálculo do RMSSD (Variabilidade da Frequência Cardíaca)
            if (intervals.length >= 2) {
                let sumSquareDiffs = 0;
                for (let k = 1; k < intervals.length; k++) {
                    const diff = intervals[k] - intervals[k - 1];
                    sumSquareDiffs += diff * diff;
                }
                hrvRmssd = parseFloat(Math.sqrt(sumSquareDiffs / (intervals.length - 1)).toFixed(1));
            }
        }

        // 3. Cálculo da Aceleração e Desaceleração Cardíaca (dBPM / dt)
        const now = performance.now();
        const dtSec = (now - this.lastBpmTime) / 1000.0;
        this.lastBpmTime = now;

        const deltaBpm = currentBpm - this.previousBpm;
        this.previousBpm = currentBpm;

        let acceleration = 0.0;
        let deceleration = 0.0;

        if (dtSec > 0) {
            const rate = deltaBpm / dtSec;
            if (rate > 0) acceleration = parseFloat(rate.toFixed(2));
            else deceleration = parseFloat(Math.abs(rate).toFixed(2));
        }

        return {
            bpm: currentBpm,
            acceleration_bpm_s: acceleration,
            deceleration_bpm_s: deceleration,
            hrv_rmssd_ms: hrvRmssd
        };
    }
}
