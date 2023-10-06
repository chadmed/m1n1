# SPDX-License-Identifier: MIT
# isp_opcodes.py: autogenerated file

CISP_CMD_START					     = 0x0000
CISP_CMD_STOP					     = 0x0001
CISP_CMD_CONFIG_GET				     = 0x0003
CISP_CMD_PRINT_ENABLE				     = 0x0004
CISP_CMD_BUILDINFO				     = 0x0006
CISP_CMD_SET_ISP_PMU_BASE			     = 0x0011
CISP_CMD_RPC_ENABLE				     = 0x0013
CISP_CMD_PMP_CTRL_SET				     = 0x001c
CISP_CMD_TRACE_ENABLE				     = 0x001d
CISP_CMD_FLICKER_SENSOR_SET              = 0x0024
CISP_CMD_CH_START				     = 0x0100
CISP_CMD_CH_STOP				     = 0x0101
CISP_CMD_CH_BUFFER_RETURN			     = 0x0104
CISP_CMD_CH_CAMERA_CONFIG_CURRENT_GET		     = 0x0105
CISP_CMD_CH_CAMERA_CONFIG_GET			     = 0x0106
CISP_CMD_CH_CAMERA_CONFIG_SELECT		     = 0x0107
CISP_CMD_CH_INFO_GET				     = 0x010d
CISP_CMD_CH_BUFFER_RECYCLE_MODE_SET		     = 0x010e
CISP_CMD_CH_BUFFER_RECYCLE_START		     = 0x010f
CISP_CMD_CH_BUFFER_RECYCLE_STOP			     = 0x0110
CISP_CMD_CH_SET_FILE_LOAD			     = 0x0111
CISP_CMD_CH_SIF_PIXEL_FORMAT_SET		     = 0x0115
CISP_CMD_CH_BUFFER_POOL_CONFIG_GET		     = 0x0116
CISP_CMD_CH_BUFFER_POOL_CONFIG_SET		     = 0x0117
CISP_CMD_CH_CAMERA_MIPI_FREQUENCY_GET		     = 0x011a
CISP_CMD_CH_CAMERA_PIX_FREQUENCY_GET		     = 0x011f
CISP_CMD_CH_CAMERA_MIPI_FREQUENCY_TOTAL_GET	     = 0x0133
CISP_CMD_CH_SBS_ENABLE				     = 0x013b
CISP_CMD_CH_LSC_POLYNOMIAL_COEFF_GET		     = 0x0142
CISP_CMD_CH_CAMERA_AGILE_FREQ_ARRAY_CURRENT_GET	     = 0x015e
CISP_CMD_CH_AE_START				     = 0x0200
CISP_CMD_CH_AE_STOP				     = 0x0201
CISP_CMD_CH_AE_FRAME_RATE_MAX_SET		     = 0x0208
CISP_CMD_CH_AE_FRAME_RATE_MIN_SET		     = 0x020a
CISP_CMD_CH_AE_STABILITY_SET			     = 0x021a
CISP_CMD_CH_AE_STABILITY_TO_STABLE_SET		     = 0x0229
CISP_CMD_CH_SENSOR_NVM_GET			     = 0x0501
CISP_CMD_CH_SENSOR_PERMODULE_LSC_INFO_GET	     = 0x0507
CISP_CMD_CH_SENSOR_PERMODULE_LSC_GRID_GET	     = 0x0511
CISP_CMD_CH_LPDP_HS_RECEIVER_TUNING_SET          = 0x051b
CISP_CMD_CH_FOCUS_LIMITS_GET			     = 0x0701
CISP_CMD_CH_CROP_SET				     = 0x0801
CISP_CMD_CH_CROP_SCL1_SET                = 0x080c
CISP_CMD_CH_CNR_START				     = 0x0a2f
CISP_CMD_CH_MBNR_ENABLE				     = 0x0a3a
CISP_CMD_CH_OUTPUT_CONFIG_SET			     = 0x0b01
CISP_CMD_CH_OUTPUT_CONFIG_SCL1_SET		     = 0x0b09
CISP_CMD_CH_PREVIEW_STREAM_SET			     = 0x0b0d
CISP_CMD_CH_FACE_DETECTION_START		     = 0x0d00
CISP_CMD_CH_FACE_DETECTION_CONFIG_GET		     = 0x0d02
CISP_CMD_CH_FACE_DETECTION_CONFIG_SET		     = 0x0d03
CISP_CMD_CH_FACE_DETECTION_ENABLE		     = 0x0d05
CISP_CMD_CH_FID_START				     = 0x3000
CISP_CMD_CH_FID_STOP				     = 0x3001
CISP_CMD_IPC_ENDPOINT_SET2			     = 0x300c
CISP_CMD_IPC_ENDPOINT_UNSET2			     = 0x300d
CISP_CMD_SET_DSID_CLR_REG_BASE2			     = 0x3204
CISP_CMD_APPLE_CH_AE_METERING_MODE_SET		     = 0x8206
CISP_CMD_APPLE_CH_AE_FD_SCENE_METERING_CONFIG_SET    = 0x820e
CISP_CMD_APPLE_CH_AE_FLICKER_FREQ_UPDATE_CURRENT_SET = 0x8212
CISP_CMD_APPLE_CH_TEMPORAL_FILTER_START		     = 0xc100
CISP_CMD_APPLE_CH_MOTION_HISTORY_START		     = 0xc102
CISP_CMD_APPLE_CH_TEMPORAL_FILTER_ENABLE	     = 0xc113

opcode_dict = {
    0x0000 : "CISP_CMD_START",
    0x0001 : "CISP_CMD_STOP",
    0x0003 : "CISP_CMD_CONFIG_GET",
    0x0004 : "CISP_CMD_PRINT_ENABLE",
    0x0006 : "CISP_CMD_BUILDINFO",
    0x0011 : "CISP_CMD_SET_ISP_PMU_BASE",
    0x0013 : "CISP_CMD_RPC_ENABLE",
    0x001c : "CISP_CMD_PMP_CTRL_SET",
    0x001d : "CISP_CMD_TRACE_ENABLE",
    0x0024 : "CISP_CMD_FLICKER_SENSOR_SET",
    0x0100 : "CISP_CMD_CH_START",
    0x0101 : "CISP_CMD_CH_STOP",
    0x0104 : "CISP_CMD_CH_BUFFER_RETURN",
    0x0105 : "CISP_CMD_CH_CAMERA_CONFIG_CURRENT_GET",
    0x0106 : "CISP_CMD_CH_CAMERA_CONFIG_GET",
    0x0107 : "CISP_CMD_CH_CAMERA_CONFIG_SELECT",
    0x010d : "CISP_CMD_CH_INFO_GET",
    0x010e : "CISP_CMD_CH_BUFFER_RECYCLE_MODE_SET",
    0x010f : "CISP_CMD_CH_BUFFER_RECYCLE_START",
    0x0110 : "CISP_CMD_CH_BUFFER_RECYCLE_STOP",
    0x0111 : "CISP_CMD_CH_SET_FILE_LOAD",
    0x0115 : "CISP_CMD_CH_SIF_PIXEL_FORMAT_SET",
    0x0116 : "CISP_CMD_CH_BUFFER_POOL_CONFIG_GET",
    0x0117 : "CISP_CMD_CH_BUFFER_POOL_CONFIG_SET",
    0x011a : "CISP_CMD_CH_CAMERA_MIPI_FREQUENCY_GET",
    0x011f : "CISP_CMD_CH_CAMERA_PIX_FREQUENCY_GET",
    0x0133 : "CISP_CMD_CH_CAMERA_MIPI_FREQUENCY_TOTAL_GET",
    0x013b : "CISP_CMD_CH_SBS_ENABLE",
    0x0142 : "CISP_CMD_CH_LSC_POLYNOMIAL_COEFF_GET",
    0x015e : "CISP_CMD_CH_CAMERA_AGILE_FREQ_ARRAY_CURRENT_GET",
    0x0200 : "CISP_CMD_CH_AE_START",
    0x0201 : "CISP_CMD_CH_AE_STOP",
    0x0208 : "CISP_CMD_CH_AE_FRAME_RATE_MAX_SET",
    0x020a : "CISP_CMD_CH_AE_FRAME_RATE_MIN_SET",
    0x021a : "CISP_CMD_CH_AE_STABILITY_SET",
    0x0229 : "CISP_CMD_CH_AE_STABILITY_TO_STABLE_SET",
    0x0501 : "CISP_CMD_CH_SENSOR_NVM_GET",
    0x0507 : "CISP_CMD_CH_SENSOR_PERMODULE_LSC_INFO_GET",
    0x0511 : "CISP_CMD_CH_SENSOR_PERMODULE_LSC_GRID_GET",
    0x051b : "CISP_CMD_CH_LPDP_HS_RECEIVER_TUNING_SET",
    0x0701 : "CISP_CMD_CH_FOCUS_LIMITS_GET",
    0x0801 : "CISP_CMD_CH_CROP_SET",
    0x080c : "CISP_CMD_CH_CROP_SCL1_SET",
    0x0a2f : "CISP_CMD_CH_CNR_START",
    0x0a3a : "CISP_CMD_CH_MBNR_ENABLE",
    0x0b01 : "CISP_CMD_CH_OUTPUT_CONFIG_SET",
    0x0b09 : "CISP_CMD_CH_OUTPUT_CONFIG_SCL1_SET",
    0x0b0d : "CISP_CMD_CH_PREVIEW_STREAM_SET",
    0x0d00 : "CISP_CMD_CH_FACE_DETECTION_START",
    0x0d02 : "CISP_CMD_CH_FACE_DETECTION_CONFIG_GET",
    0x0d03 : "CISP_CMD_CH_FACE_DETECTION_CONFIG_SET",
    0x0d05 : "CISP_CMD_CH_FACE_DETECTION_ENABLE",
    0x3000 : "CISP_CMD_CH_FID_START",
    0x3001 : "CISP_CMD_CH_FID_STOP",
    0x300c : "CISP_CMD_IPC_ENDPOINT_SET2",
    0x300d : "CISP_CMD_IPC_ENDPOINT_UNSET2",
    0x3204 : "CISP_CMD_SET_DSID_CLR_REG_BASE2",
    0x8206 : "CISP_CMD_APPLE_CH_AE_METERING_MODE_SET",
    0x820e : "CISP_CMD_APPLE_CH_AE_FD_SCENE_METERING_CONFIG_SET",
    0x8212 : "CISP_CMD_APPLE_CH_AE_FLICKER_FREQ_UPDATE_CURRENT_SET",
    0xc100 : "CISP_CMD_APPLE_CH_TEMPORAL_FILTER_START",
    0xc102 : "CISP_CMD_APPLE_CH_MOTION_HISTORY_START",
    0xc113 : "CISP_CMD_APPLE_CH_TEMPORAL_FILTER_ENABLE",
}