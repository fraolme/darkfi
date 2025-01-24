/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2025 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{
    mesh::Color,
    ui::{ShapeVertex, VectorShape},
};
pub fn create_netlogo2(color: Color) -> VectorShape {
    VectorShape {
        verts: vec![
            ShapeVertex::from_xy(-0.092672, 0.229295, color),
            ShapeVertex::from_xy(-0.05587, 0.215822, color),
            ShapeVertex::from_xy(-0.094244, 0.213289, color),
            ShapeVertex::from_xy(-0.057849, 0.2006, color),
            ShapeVertex::from_xy(-0.096923, 0.204068, color),
            ShapeVertex::from_xy(-0.064964, 0.176295, color),
            ShapeVertex::from_xy(-0.106859, 0.17437, color),
            ShapeVertex::from_xy(-0.080393, 0.143705, color),
            ShapeVertex::from_xy(-0.125286, 0.144779, color),
            ShapeVertex::from_xy(-0.102744, 0.112396, color),
            ShapeVertex::from_xy(-0.143901, 0.124958, color),
            ShapeVertex::from_xy(-0.122513, 0.09335, color),
            ShapeVertex::from_xy(-0.164119, 0.112271, color),
            ShapeVertex::from_xy(-0.130064, 0.088085, color),
            ShapeVertex::from_xy(-0.175551, 0.030293, color),
            ShapeVertex::from_xy(-0.134894, 0.044002, color),
            ShapeVertex::from_xy(-0.146567, 0.004396, color),
            ShapeVertex::from_xy(-0.100873, 0.013953, color),
            ShapeVertex::from_xy(-0.180836, -0.073298, color),
            ShapeVertex::from_xy(-0.146544, -0.090607, color),
            ShapeVertex::from_xy(-0.194759, -0.099656, color),
            ShapeVertex::from_xy(-0.161785, -0.118893, color),
            ShapeVertex::from_xy(-0.24534, -0.168477, color),
            ShapeVertex::from_xy(-0.230976, -0.214723, color),
            ShapeVertex::from_xy(-0.283526, -0.156565, color),
            ShapeVertex::from_xy(-0.276551, -0.199385, color),
            ShapeVertex::from_xy(-0.345759, -0.206449, color),
            ShapeVertex::from_xy(-0.308696, -0.224025, color),
            ShapeVertex::from_xy(-0.347698, -0.230779, color),
            ShapeVertex::from_xy(-0.31187, -0.248355, color),
            ShapeVertex::from_xy(-0.351577, -0.248057, color),
            ShapeVertex::from_xy(-0.317952, -0.268807, color),
            ShapeVertex::from_xy(-0.364359, -0.275914, color),
            ShapeVertex::from_xy(-0.330734, -0.296664, color),
            ShapeVertex::from_xy(-0.371588, -0.288255, color),
            ShapeVertex::from_xy(-0.338756, -0.309005, color),
            ShapeVertex::from_xy(-0.383991, -0.305137, color),
            ShapeVertex::from_xy(-0.352623, -0.327738, color),
            ShapeVertex::from_xy(-0.396521, -0.318724, color),
            ShapeVertex::from_xy(-0.366992, -0.343165, color),
            ShapeVertex::from_xy(-0.413433, -0.328758, color),
            ShapeVertex::from_xy(-0.384904, -0.357928, color),
            ShapeVertex::from_xy(-0.430432, -0.355343, color),
            ShapeVertex::from_xy(-0.399449, -0.37838, color),
            ShapeVertex::from_xy(-0.44753, -0.372364, color),
            ShapeVertex::from_xy(-0.415934, -0.396628, color),
            ShapeVertex::from_xy(-0.459676, -0.378726, color),
            ShapeVertex::from_xy(-0.425014, -0.405443, color),
            ShapeVertex::from_xy(-0.464995, -0.39139, color),
            ShapeVertex::from_xy(-0.43524, -0.413201, color),
            ShapeVertex::from_xy(-0.47971, -0.410527, color),
            ShapeVertex::from_xy(-0.448727, -0.433564, color),
            ShapeVertex::from_xy(-0.488437, -0.421987, color),
            ShapeVertex::from_xy(-0.457455, -0.445024, color),
            ShapeVertex::from_xy(-0.501131, -0.432765, color),
            ShapeVertex::from_xy(-0.470149, -0.459482, color),
            ShapeVertex::from_xy(-0.512775, -0.443347, color),
            ShapeVertex::from_xy(-0.484188, -0.46948, color),
            ShapeVertex::from_xy(-0.521671, -0.453899, color),
            ShapeVertex::from_xy(-0.529957, -0.469767, color),
            ShapeVertex::from_xy(-0.545649, -0.491364, color),
            ShapeVertex::from_xy(-0.538076, -0.514138, color),
            ShapeVertex::from_xy(-0.560006, -0.502461, color),
            ShapeVertex::from_xy(-0.466747, -0.465245, color),
            ShapeVertex::from_xy(-0.599063, -0.537151, color),
            ShapeVertex::from_xy(-0.520701, -0.511463, color),
            ShapeVertex::from_xy(-0.476964, -0.50893, color),
            ShapeVertex::from_xy(-0.49228, -0.508132, color),
            ShapeVertex::from_xy(-0.459951, -0.501965, color),
            ShapeVertex::from_xy(-0.450809, -0.460288, color),
            ShapeVertex::from_xy(-0.448024, -0.497133, color),
            ShapeVertex::from_xy(-0.435296, -0.457325, color),
            ShapeVertex::from_xy(-0.434084, -0.494278, color),
            ShapeVertex::from_xy(-0.418687, -0.456091, color),
            ShapeVertex::from_xy(-0.416189, -0.493308, color),
            ShapeVertex::from_xy(-0.404082, -0.456091, color),
            ShapeVertex::from_xy(-0.402966, -0.493132, color),
            ShapeVertex::from_xy(-0.386972, -0.44851, color),
            ShapeVertex::from_xy(-0.3975, -0.493661, color),
            ShapeVertex::from_xy(-0.379428, -0.48608, color),
            ShapeVertex::from_xy(-0.372649, -0.443799, color),
            ShapeVertex::from_xy(-0.372641, -0.483984, color),
            ShapeVertex::from_xy(-0.357376, -0.441243, color),
            ShapeVertex::from_xy(-0.355892, -0.481515, color),
            ShapeVertex::from_xy(-0.344105, -0.440537, color),
            ShapeVertex::from_xy(-0.34117, -0.480545, color),
            ShapeVertex::from_xy(-0.335425, -0.440449, color),
            ShapeVertex::from_xy(-0.330944, -0.480369, color),
            ShapeVertex::from_xy(-0.321737, -0.433397, color),
            ShapeVertex::from_xy(-0.316487, -0.473317, color),
            ShapeVertex::from_xy(-0.302361, -0.427576, color),
            ShapeVertex::from_xy(-0.298518, -0.467148, color),
            ShapeVertex::from_xy(-0.279061, -0.424491, color),
            ShapeVertex::from_xy(-0.273908, -0.464063, color),
            ShapeVertex::from_xy(-0.248041, -0.424711, color),
            ShapeVertex::from_xy(-0.246066, -0.46443, color),
            ShapeVertex::from_xy(-0.217119, -0.429439, color),
            ShapeVertex::from_xy(-0.222485, -0.468103, color),
            ShapeVertex::from_xy(-0.197143, -0.435977, color),
            ShapeVertex::from_xy(-0.20221, -0.474274, color),
            ShapeVertex::from_xy(-0.183152, -0.441927, color),
            ShapeVertex::from_xy(-0.187558, -0.481464, color),
            ShapeVertex::from_xy(-0.17624, -0.438426, color),
            ShapeVertex::from_xy(-0.171396, -0.478305, color),
            ShapeVertex::from_xy(-0.170254, -0.434753, color),
            ShapeVertex::from_xy(-0.163683, -0.473677, color),
            ShapeVertex::from_xy(-0.160381, -0.430713, color),
            ShapeVertex::from_xy(-0.14809, -0.467082, color),
            ShapeVertex::from_xy(-0.153964, -0.428729, color),
            ShapeVertex::from_xy(-0.134279, -0.463189, color),
            ShapeVertex::from_xy(-0.14502, -0.424692, color),
            ShapeVertex::from_xy(-0.122011, -0.456063, color),
            ShapeVertex::from_xy(-0.136505, -0.377442, color),
            ShapeVertex::from_xy(-0.111433, -0.451729, color),
            ShapeVertex::from_xy(-0.103198, -0.412567, color),
            ShapeVertex::from_xy(-0.036971, -0.367114, color),
            ShapeVertex::from_xy(-0.025071, -0.404793, color),
            ShapeVertex::from_xy(0.012798, -0.363926, color),
            ShapeVertex::from_xy(0.011879, -0.403525, color),
            ShapeVertex::from_xy(0.125515, -0.377896, color),
            ShapeVertex::from_xy(0.093843, -0.412555, color),
            ShapeVertex::from_xy(0.134011, -0.422327, color),
            ShapeVertex::from_xy(0.101743, -0.451347, color),
            ShapeVertex::from_xy(0.142949, -0.427722, color),
            ShapeVertex::from_xy(0.11803, -0.458503, color),
            ShapeVertex::from_xy(0.145005, -0.428569, color),
            ShapeVertex::from_xy(0.125752, -0.463793, color),
            ShapeVertex::from_xy(0.155283, -0.432483, color),
            ShapeVertex::from_xy(0.138447, -0.467495, color),
            ShapeVertex::from_xy(0.164579, -0.437031, color),
            ShapeVertex::from_xy(0.149448, -0.472044, color),
            ShapeVertex::from_xy(0.173333, -0.440313, color),
            ShapeVertex::from_xy(0.156536, -0.47564, color),
            ShapeVertex::from_xy(0.160344, -0.478074, color),
            ShapeVertex::from_xy(0.168913, -0.479237, color),
            ShapeVertex::from_xy(0.185969, -0.435614, color),
            ShapeVertex::from_xy(0.178537, -0.481917, color),
            ShapeVertex::from_xy(0.19162, -0.47466, color),
            ShapeVertex::from_xy(0.197624, -0.431639, color),
            ShapeVertex::from_xy(0.200895, -0.470241, color),
            ShapeVertex::from_xy(0.213354, -0.427514, color),
            ShapeVertex::from_xy(0.219513, -0.466116, color),
            ShapeVertex::from_xy(0.229496, -0.42515, color),
            ShapeVertex::from_xy(0.235106, -0.463825, color),
            ShapeVertex::from_xy(0.268793, -0.424749, color),
            ShapeVertex::from_xy(0.267581, -0.464036, color),
            ShapeVertex::from_xy(0.296223, -0.429278, color),
            ShapeVertex::from_xy(0.286088, -0.466299, color),
            ShapeVertex::from_xy(0.316019, -0.436803, color),
            ShapeVertex::from_xy(0.297929, -0.471256, color),
            ShapeVertex::from_xy(0.323947, -0.439875, color),
            ShapeVertex::from_xy(0.314326, -0.478978, color),
            ShapeVertex::from_xy(0.343678, -0.441016, color),
            ShapeVertex::from_xy(0.328659, -0.478191, color),
            ShapeVertex::from_xy(0.360488, -0.44292, color),
            ShapeVertex::from_xy(0.345373, -0.480095, color),
            ShapeVertex::from_xy(0.375495, -0.448136, color),
            ShapeVertex::from_xy(0.360183, -0.483586, color),
            ShapeVertex::from_xy(0.385027, -0.451278, color),
            ShapeVertex::from_xy(0.374675, -0.489192, color),
            ShapeVertex::from_xy(0.393009, -0.454805, color),
            ShapeVertex::from_xy(0.38536, -0.493741, color),
            ShapeVertex::from_xy(0.410295, -0.455086, color),
            ShapeVertex::from_xy(0.394919, -0.493283, color),
            ShapeVertex::from_xy(0.422436, -0.455897, color),
            ShapeVertex::from_xy(0.405921, -0.4936, color),
            ShapeVertex::from_xy(0.438469, -0.4584, color),
            ShapeVertex::from_xy(0.423376, -0.49561, color),
            ShapeVertex::from_xy(0.450072, -0.462207, color),
            ShapeVertex::from_xy(0.437974, -0.498678, color),
            ShapeVertex::from_xy(0.463408, -0.46718, color),
            ShapeVertex::from_xy(0.453101, -0.50439, color),
            ShapeVertex::from_xy(0.471083, -0.46856, color),
            ShapeVertex::from_xy(0.465372, -0.509468, color),
            ShapeVertex::from_xy(0.474364, -0.508516, color),
            ShapeVertex::from_xy(0.488116, -0.50841, color),
            ShapeVertex::from_xy(0.507001, -0.45051, color),
            ShapeVertex::from_xy(0.503349, -0.509256, color),
            ShapeVertex::from_xy(0.539638, -0.493907, color),
            ShapeVertex::from_xy(0.548408, -0.521091, color),
            ShapeVertex::from_xy(0.566392, -0.516754, color),
            ShapeVertex::from_xy(0.568084, -0.528496, color),
            ShapeVertex::from_xy(0.512925, -0.46204, color),
            ShapeVertex::from_xy(0.587126, -0.534737, color),
            ShapeVertex::from_xy(0.527798, -0.481394, color),
            ShapeVertex::from_xy(0.466442, -0.461912, color),
            ShapeVertex::from_xy(0.501552, -0.441158, color),
            ShapeVertex::from_xy(0.443865, -0.445315, color),
            ShapeVertex::from_xy(0.483799, -0.426483, color),
            ShapeVertex::from_xy(0.442974, -0.44388, color),
            ShapeVertex::from_xy(0.470232, -0.411259, color),
            ShapeVertex::from_xy(0.443656, -0.439267, color),
            ShapeVertex::from_xy(0.45835, -0.395287, color),
            ShapeVertex::from_xy(0.423418, -0.414497, color),
            ShapeVertex::from_xy(0.452744, -0.386824, color),
            ShapeVertex::from_xy(0.418975, -0.405823, color),
            ShapeVertex::from_xy(0.449147, -0.379736, color),
            ShapeVertex::from_xy(0.411177, -0.400882, color),
            ShapeVertex::from_xy(0.438145, -0.371591, color),
            ShapeVertex::from_xy(0.401022, -0.391784, color),
            ShapeVertex::from_xy(0.42799, -0.362493, color),
            ShapeVertex::from_xy(0.391607, -0.381523, color),
            ShapeVertex::from_xy(0.418575, -0.352232, color),
            ShapeVertex::from_xy(0.383567, -0.371791, color),
            ShapeVertex::from_xy(0.410535, -0.3425, color),
            ShapeVertex::from_xy(0.377432, -0.362905, color),
            ShapeVertex::from_xy(0.407075, -0.336431, color),
            ShapeVertex::from_xy(0.373941, -0.357616, color),
            ShapeVertex::from_xy(0.402985, -0.330437, color),
            ShapeVertex::from_xy(0.37172, -0.35349, color),
            ShapeVertex::from_xy(0.400729, -0.32522, color),
            ShapeVertex::from_xy(0.364209, -0.348413, color),
            ShapeVertex::from_xy(0.390754, -0.31891, color),
            ShapeVertex::from_xy(0.355535, -0.341219, color),
            ShapeVertex::from_xy(0.382079, -0.311717, color),
            ShapeVertex::from_xy(0.346754, -0.332545, color),
            ShapeVertex::from_xy(0.373299, -0.303042, color),
            ShapeVertex::from_xy(0.338883, -0.323098, color),
            ShapeVertex::from_xy(0.365831, -0.293871, color),
            ShapeVertex::from_xy(0.328304, -0.309558, color),
            ShapeVertex::from_xy(0.358723, -0.284034, color),
            ShapeVertex::from_xy(0.317862, -0.292809, color),
            ShapeVertex::from_xy(0.351571, -0.271632, color),
            ShapeVertex::from_xy(0.306149, -0.268144, color),
            ShapeVertex::from_xy(0.342953, -0.251602, color),
            ShapeVertex::from_xy(0.298264, -0.241635, color),
            ShapeVertex::from_xy(0.33755, -0.23293, color),
            ShapeVertex::from_xy(0.297205, -0.224257, color),
            ShapeVertex::from_xy(0.33618, -0.220715, color),
            ShapeVertex::from_xy(0.336474, -0.205485, color),
            ShapeVertex::from_xy(0.262908, -0.197561, color),
            ShapeVertex::from_xy(0.27111, -0.154727, color),
            ShapeVertex::from_xy(0.218121, -0.213047, color),
            ShapeVertex::from_xy(0.234394, -0.166762, color),
            ShapeVertex::from_xy(0.163183, -0.137412, color),
            ShapeVertex::from_xy(0.2138, -0.138901, color),
            ShapeVertex::from_xy(0.143859, -0.108768, color),
            ShapeVertex::from_xy(0.185135, -0.098505, color),
            ShapeVertex::from_xy(0.128552, -0.076177, color),
            ShapeVertex::from_xy(0.173508, -0.082841, color),
            ShapeVertex::from_xy(0.088385, 0.015481, color),
            ShapeVertex::from_xy(0.134512, 0.003277, color),
            ShapeVertex::from_xy(0.123167, 0.047775, color),
            ShapeVertex::from_xy(0.163583, 0.031179, color),
            ShapeVertex::from_xy(0.117501, 0.088977, color),
            ShapeVertex::from_xy(0.151972, 0.113553, color),
            ShapeVertex::from_xy(0.107551, 0.095682, color),
            ShapeVertex::from_xy(0.13681, 0.122953, color),
            ShapeVertex::from_xy(0.092419, 0.109526, color),
            ShapeVertex::from_xy(0.120318, 0.137052, color),
            ShapeVertex::from_xy(0.076275, 0.12943, color),
            ShapeVertex::from_xy(0.104736, 0.158943, color),
            ShapeVertex::from_xy(0.059508, 0.158331, color),
            ShapeVertex::from_xy(0.090712, 0.185717, color),
            ShapeVertex::from_xy(0.046665, 0.194011, color),
            ShapeVertex::from_xy(0.083311, 0.215081, color),
            ShapeVertex::from_xy(0.044018, 0.210065, color),
            ShapeVertex::from_xy(0.081321, 0.233397, color),
            ShapeVertex::from_xy(0.042487, 0.219718, color),
            ShapeVertex::from_xy(0.074196, 0.243074, color),
            ShapeVertex::from_xy(0.034942, 0.235213, color),
            ShapeVertex::from_xy(0.069438, 0.253746, color),
            ShapeVertex::from_xy(0.028356, 0.254561, color),
            ShapeVertex::from_xy(0.063131, 0.275333, color),
            ShapeVertex::from_xy(0.025239, 0.264834, color),
            ShapeVertex::from_xy(0.05969, 0.296005, color),
            ShapeVertex::from_xy(0.022996, 0.277362, color),
            ShapeVertex::from_xy(0.058158, 0.30008, color),
            ShapeVertex::from_xy(0.022203, 0.285144, color),
            ShapeVertex::from_xy(0.053695, 0.308145, color),
            ShapeVertex::from_xy(0.0143, 0.298679, color),
            ShapeVertex::from_xy(0.045487, 0.324373, color),
            ShapeVertex::from_xy(0.008279, 0.313245, color),
            ShapeVertex::from_xy(0.040425, 0.341506, color),
            ShapeVertex::from_xy(0.00288, 0.328564, color),
            ShapeVertex::from_xy(0.035676, 0.358218, color),
            ShapeVertex::from_xy(-0.001588, 0.344935, color),
            ShapeVertex::from_xy(0.032728, 0.376588, color),
            ShapeVertex::from_xy(-0.005301, 0.360096, color),
            ShapeVertex::from_xy(0.025845, 0.384946, color),
            ShapeVertex::from_xy(0.015738, 0.406389, color),
            ShapeVertex::from_xy(-0.005606, 0.496526, color),
            ShapeVertex::from_xy(0.008579, 0.424158, color),
            ShapeVertex::from_xy(-0.013808, 0.456325, color),
            ShapeVertex::from_xy(0.003633, 0.450925, color),
            ShapeVertex::from_xy(-0.029276, 0.399374, color),
            ShapeVertex::from_xy(-0.020921, 0.419415, color),
            ShapeVertex::from_xy(-0.043889, 0.371874, color),
            ShapeVertex::from_xy(-0.007666, 0.355748, color),
            ShapeVertex::from_xy(-0.045644, 0.360687, color),
            ShapeVertex::from_xy(-0.010079, 0.342147, color),
            ShapeVertex::from_xy(-0.048057, 0.347087, color),
            ShapeVertex::from_xy(-0.014247, 0.329644, color),
            ShapeVertex::from_xy(-0.055449, 0.325642, color),
            ShapeVertex::from_xy(-0.019511, 0.315606, color),
            ShapeVertex::from_xy(-0.025653, 0.302006, color),
            ShapeVertex::from_xy(-0.062448, 0.311381, color),
            ShapeVertex::from_xy(-0.030479, 0.292793, color),
            ShapeVertex::from_xy(-0.067274, 0.302168, color),
            ShapeVertex::from_xy(-0.03355, 0.288625, color),
            ShapeVertex::from_xy(-0.070345, 0.298, color),
            ShapeVertex::from_xy(-0.034427, 0.279412, color),
            ShapeVertex::from_xy(-0.071223, 0.288788, color),
            ShapeVertex::from_xy(-0.035963, 0.268663, color),
            ShapeVertex::from_xy(-0.072624, 0.280155, color),
            ShapeVertex::from_xy(-0.040131, 0.253967, color),
            ShapeVertex::from_xy(-0.044079, 0.242341, color),
            ShapeVertex::from_xy(-0.074367, 0.269462, color),
            ShapeVertex::from_xy(-0.049553, 0.228522, color),
            ShapeVertex::from_xy(-0.080433, 0.251502, color),
        ],
        indices: vec![
            1, 2, 0, 3, 4, 2, 5, 6, 4, 7, 8, 6, 9, 10, 8, 10, 13, 12, 13, 14, 12, 15, 16, 14, 16,
            19, 18, 19, 20, 18, 21, 22, 20, 22, 25, 24, 25, 26, 24, 27, 28, 26, 29, 30, 28, 31, 32,
            30, 33, 34, 32, 34, 37, 36, 36, 39, 38, 38, 41, 40, 41, 42, 40, 42, 45, 44, 44, 47, 46,
            46, 49, 48, 49, 50, 48, 50, 53, 52, 52, 55, 54, 54, 57, 56, 56, 57, 58, 58, 57, 59, 59,
            57, 60, 60, 57, 62, 62, 57, 64, 64, 57, 61, 61, 57, 65, 65, 57, 67, 67, 57, 66, 57, 68,
            66, 63, 70, 68, 69, 72, 70, 71, 74, 72, 74, 75, 76, 76, 75, 78, 75, 79, 78, 77, 81, 79,
            80, 83, 81, 82, 85, 83, 85, 86, 87, 86, 89, 87, 88, 91, 89, 90, 93, 91, 93, 94, 95, 95,
            96, 97, 97, 98, 99, 99, 100, 101, 100, 103, 101, 102, 105, 103, 104, 107, 105, 107,
            108, 109, 109, 110, 111, 111, 110, 113, 110, 114, 113, 114, 115, 116, 116, 117, 118,
            117, 120, 118, 120, 121, 122, 121, 124, 122, 123, 126, 124, 125, 128, 126, 127, 130,
            128, 129, 132, 130, 132, 131, 133, 133, 131, 134, 134, 131, 136, 131, 137, 136, 135,
            139, 137, 138, 141, 139, 141, 142, 143, 143, 144, 145, 144, 147, 145, 147, 148, 149,
            149, 150, 151, 150, 153, 151, 152, 155, 153, 154, 157, 155, 157, 158, 159, 159, 160,
            161, 160, 163, 161, 162, 165, 163, 164, 167, 165, 166, 169, 167, 169, 170, 171, 171,
            172, 173, 173, 172, 174, 174, 172, 175, 175, 172, 177, 177, 172, 179, 179, 172, 181,
            181, 172, 183, 183, 172, 180, 180, 172, 178, 178, 172, 184, 184, 172, 182, 172, 176,
            182, 185, 186, 176, 187, 188, 186, 188, 191, 190, 191, 192, 190, 193, 194, 192, 194,
            195, 196, 195, 198, 196, 197, 200, 198, 200, 201, 202, 202, 203, 204, 204, 205, 206,
            206, 207, 208, 208, 209, 210, 209, 212, 210, 211, 214, 212, 214, 215, 216, 216, 217,
            218, 218, 219, 220, 220, 221, 222, 222, 223, 224, 224, 225, 226, 226, 227, 228, 228,
            227, 229, 229, 230, 231, 230, 233, 231, 232, 235, 233, 234, 237, 235, 236, 239, 237,
            238, 241, 239, 241, 242, 243, 242, 245, 243, 244, 247, 245, 246, 249, 247, 249, 250,
            251, 251, 252, 253, 253, 254, 255, 255, 256, 257, 257, 258, 259, 259, 260, 261, 261,
            262, 263, 263, 264, 265, 265, 266, 267, 267, 268, 269, 269, 270, 271, 271, 272, 273,
            273, 274, 275, 275, 276, 277, 277, 278, 279, 279, 278, 280, 280, 278, 282, 282, 278,
            284, 284, 278, 281, 281, 278, 283, 283, 278, 286, 286, 278, 285, 285, 278, 287, 287,
            288, 289, 289, 290, 291, 291, 292, 293, 293, 292, 294, 293, 295, 296, 296, 297, 298,
            298, 299, 300, 299, 302, 300, 301, 304, 302, 304, 303, 305, 305, 307, 304, 306, 309,
            307, 308, 0, 309, 1, 3, 2, 3, 5, 4, 5, 7, 6, 7, 9, 8, 9, 11, 10, 10, 11, 13, 13, 15,
            14, 15, 17, 16, 16, 17, 19, 19, 21, 20, 21, 23, 22, 22, 23, 25, 25, 27, 26, 27, 29, 28,
            29, 31, 30, 31, 33, 32, 33, 35, 34, 34, 35, 37, 36, 37, 39, 38, 39, 41, 41, 43, 42, 42,
            43, 45, 44, 45, 47, 46, 47, 49, 49, 51, 50, 50, 51, 53, 52, 53, 55, 54, 55, 57, 57, 63,
            68, 63, 69, 70, 69, 71, 72, 71, 73, 74, 74, 73, 75, 75, 77, 79, 77, 80, 81, 80, 82, 83,
            82, 84, 85, 85, 84, 86, 86, 88, 89, 88, 90, 91, 90, 92, 93, 93, 92, 94, 95, 94, 96, 97,
            96, 98, 99, 98, 100, 100, 102, 103, 102, 104, 105, 104, 106, 107, 107, 106, 108, 109,
            108, 110, 110, 112, 114, 114, 112, 115, 116, 115, 117, 117, 119, 120, 120, 119, 121,
            121, 123, 124, 123, 125, 126, 125, 127, 128, 127, 129, 130, 129, 131, 132, 131, 135,
            137, 135, 138, 139, 138, 140, 141, 141, 140, 142, 143, 142, 144, 144, 146, 147, 147,
            146, 148, 149, 148, 150, 150, 152, 153, 152, 154, 155, 154, 156, 157, 157, 156, 158,
            159, 158, 160, 160, 162, 163, 162, 164, 165, 164, 166, 167, 166, 168, 169, 169, 168,
            170, 171, 170, 172, 172, 185, 176, 185, 187, 186, 187, 189, 188, 188, 189, 191, 191,
            193, 192, 193, 195, 194, 195, 197, 198, 197, 199, 200, 200, 199, 201, 202, 201, 203,
            204, 203, 205, 206, 205, 207, 208, 207, 209, 209, 211, 212, 211, 213, 214, 214, 213,
            215, 216, 215, 217, 218, 217, 219, 220, 219, 221, 222, 221, 223, 224, 223, 225, 226,
            225, 227, 229, 227, 230, 230, 232, 233, 232, 234, 235, 234, 236, 237, 236, 238, 239,
            238, 240, 241, 241, 240, 242, 242, 244, 245, 244, 246, 247, 246, 248, 249, 249, 248,
            250, 251, 250, 252, 253, 252, 254, 255, 254, 256, 257, 256, 258, 259, 258, 260, 261,
            260, 262, 263, 262, 264, 265, 264, 266, 267, 266, 268, 269, 268, 270, 271, 270, 272,
            273, 272, 274, 275, 274, 276, 277, 276, 278, 287, 278, 288, 289, 288, 290, 291, 290,
            292, 293, 294, 295, 296, 295, 297, 298, 297, 299, 299, 301, 302, 301, 303, 304, 305,
            306, 307, 306, 308, 309, 308, 1, 0,
        ],
    }
}
