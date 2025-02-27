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
pub fn create_netlogo3(color: Color) -> VectorShape {
    VectorShape {
        verts: vec![
            ShapeVertex::from_xy(-0.142341, 0.515982, color),
            ShapeVertex::from_xy(-0.101938, 0.504233, color),
            ShapeVertex::from_xy(-0.144884, 0.490086, color),
            ShapeVertex::from_xy(-0.105618, 0.478168, color),
            ShapeVertex::from_xy(-0.155439, 0.451718, color),
            ShapeVertex::from_xy(-0.116172, 0.4398, color),
            ShapeVertex::from_xy(-0.177259, 0.404146, color),
            ShapeVertex::from_xy(-0.141135, 0.384678, color),
            ShapeVertex::from_xy(-0.208986, 0.360096, color),
            ShapeVertex::from_xy(-0.1773, 0.334021, color),
            ShapeVertex::from_xy(-0.237192, 0.33329, color),
            ShapeVertex::from_xy(-0.209285, 0.303205, color),
            ShapeVertex::from_xy(-0.265118, 0.316591, color),
            ShapeVertex::from_xy(-0.226287, 0.29038, color),
            ShapeVertex::from_xy(-0.283232, 0.190078, color),
            ShapeVertex::from_xy(-0.238696, 0.205942, color),
            ShapeVertex::from_xy(-0.231742, 0.14301, color),
            ShapeVertex::from_xy(-0.181929, 0.154453, color),
            ShapeVertex::from_xy(-0.292931, 0.002946, color),
            ShapeVertex::from_xy(-0.2541, -0.012425, color),
            ShapeVertex::from_xy(-0.316607, -0.0397, color),
            ShapeVertex::from_xy(-0.281057, -0.061062, color),
            ShapeVertex::from_xy(-0.410458, -0.166927, color),
            ShapeVertex::from_xy(-0.394163, -0.21496, color),
            ShapeVertex::from_xy(-0.476211, -0.144676, color),
            ShapeVertex::from_xy(-0.467903, -0.190143, color),
            ShapeVertex::from_xy(-0.57591, -0.22241, color),
            ShapeVertex::from_xy(-0.534797, -0.241918, color),
            ShapeVertex::from_xy(-0.579048, -0.261776, color),
            ShapeVertex::from_xy(-0.539932, -0.281284, color),
            ShapeVertex::from_xy(-0.585323, -0.289732, color),
            ShapeVertex::from_xy(-0.549773, -0.314374, color),
            ShapeVertex::from_xy(-0.606005, -0.334803, color),
            ShapeVertex::from_xy(-0.570455, -0.359445, color),
            ShapeVertex::from_xy(-0.617701, -0.354771, color),
            ShapeVertex::from_xy(-0.583434, -0.379413, color),
            ShapeVertex::from_xy(-0.637769, -0.382085, color),
            ShapeVertex::from_xy(-0.605869, -0.409722, color),
            ShapeVertex::from_xy(-0.661018, -0.407045, color),
            ShapeVertex::from_xy(-0.629118, -0.434682, color),
            ShapeVertex::from_xy(-0.689374, -0.430227, color),
            ShapeVertex::from_xy(-0.658099, -0.458569, color),
            ShapeVertex::from_xy(-0.712908, -0.463317, color),
            ShapeVertex::from_xy(-0.681633, -0.49166, color),
            ShapeVertex::from_xy(-0.73958, -0.492841, color),
            ShapeVertex::from_xy(-0.708305, -0.521184, color),
            ShapeVertex::from_xy(-0.754271, -0.507104, color),
            ShapeVertex::from_xy(-0.722996, -0.535447, color),
            ShapeVertex::from_xy(-0.770816, -0.519656, color),
            ShapeVertex::from_xy(-0.739541, -0.547999, color),
            ShapeVertex::from_xy(-0.792639, -0.552603, color),
            ShapeVertex::from_xy(-0.761364, -0.580946, color),
            ShapeVertex::from_xy(-0.806759, -0.571146, color),
            ShapeVertex::from_xy(-0.775484, -0.599488, color),
            ShapeVertex::from_xy(-0.827298, -0.594537, color),
            ShapeVertex::from_xy(-0.796023, -0.62288, color),
            ShapeVertex::from_xy(-0.848122, -0.612651, color),
            ShapeVertex::from_xy(-0.816847, -0.640994, color),
            ShapeVertex::from_xy(-0.860531, -0.622778, color),
            ShapeVertex::from_xy(-0.827402, -0.64998, color),
            ShapeVertex::from_xy(-0.873938, -0.648451, color),
            ShapeVertex::from_xy(-0.840809, -0.673371, color),
            ShapeVertex::from_xy(-0.899327, -0.683396, color),
            ShapeVertex::from_xy(-0.888772, -0.719339, color),
            ShapeVertex::from_xy(-0.919865, -0.707215, color),
            ShapeVertex::from_xy(-0.786324, -0.663957, color),
            ShapeVertex::from_xy(-0.960943, -0.743586, color),
            ShapeVertex::from_xy(-0.858962, -0.715916, color),
            ShapeVertex::from_xy(-0.808043, -0.71777, color),
            ShapeVertex::from_xy(-0.82687, -0.715488, color),
            ShapeVertex::from_xy(-0.813852, -0.675225, color),
            ShapeVertex::from_xy(-0.825547, -0.673799, color),
            ShapeVertex::from_xy(-0.780515, -0.706502, color),
            ShapeVertex::from_xy(-0.759087, -0.655938, color),
            ShapeVertex::from_xy(-0.755264, -0.697691, color),
            ShapeVertex::from_xy(-0.732577, -0.651144, color),
            ShapeVertex::from_xy(-0.73271, -0.693072, color),
            ShapeVertex::from_xy(-0.704193, -0.649147, color),
            ShapeVertex::from_xy(-0.703756, -0.691503, color),
            ShapeVertex::from_xy(-0.679233, -0.649147, color),
            ShapeVertex::from_xy(-0.682361, -0.691217, color),
            ShapeVertex::from_xy(-0.649993, -0.636881, color),
            ShapeVertex::from_xy(-0.673518, -0.692073, color),
            ShapeVertex::from_xy(-0.644279, -0.679807, color),
            ShapeVertex::from_xy(-0.625517, -0.629259, color),
            ShapeVertex::from_xy(-0.619405, -0.671454, color),
            ShapeVertex::from_xy(-0.599415, -0.625123, color),
            ShapeVertex::from_xy(-0.592305, -0.66746, color),
            ShapeVertex::from_xy(-0.576737, -0.623982, color),
            ShapeVertex::from_xy(-0.568486, -0.665891, color),
            ShapeVertex::from_xy(-0.561904, -0.623839, color),
            ShapeVertex::from_xy(-0.551941, -0.665606, color),
            ShapeVertex::from_xy(-0.538512, -0.612428, color),
            ShapeVertex::from_xy(-0.528549, -0.654196, color),
            ShapeVertex::from_xy(-0.505399, -0.603011, color),
            ShapeVertex::from_xy(-0.499476, -0.644215, color),
            ShapeVertex::from_xy(-0.465581, -0.598019, color),
            ShapeVertex::from_xy(-0.459658, -0.639223, color),
            ShapeVertex::from_xy(-0.41257, -0.598376, color),
            ShapeVertex::from_xy(-0.414611, -0.639817, color),
            ShapeVertex::from_xy(-0.366096, -0.605032, color),
            ShapeVertex::from_xy(-0.376457, -0.64576, color),
            ShapeVertex::from_xy(-0.333172, -0.61561, color),
            ShapeVertex::from_xy(-0.343652, -0.655745, color),
            ShapeVertex::from_xy(-0.310114, -0.625238, color),
            ShapeVertex::from_xy(-0.315958, -0.66977, color),
            ShapeVertex::from_xy(-0.292879, -0.618582, color),
            ShapeVertex::from_xy(-0.289809, -0.664659, color),
            ShapeVertex::from_xy(-0.28135, -0.612639, color),
            ShapeVertex::from_xy(-0.277329, -0.657171, color),
            ShapeVertex::from_xy(-0.262332, -0.606101, color),
            ShapeVertex::from_xy(-0.256885, -0.648494, color),
            ShapeVertex::from_xy(-0.249971, -0.602892, color),
            ShapeVertex::from_xy(-0.23454, -0.642195, color),
            ShapeVertex::from_xy(-0.233925, -0.593383, color),
            ShapeVertex::from_xy(-0.21469, -0.630665, color),
            ShapeVertex::from_xy(-0.217523, -0.516933, color),
            ShapeVertex::from_xy(-0.197574, -0.623653, color),
            ShapeVertex::from_xy(-0.183454, -0.555903, color),
            ShapeVertex::from_xy(-0.033272, -0.496651, color),
            ShapeVertex::from_xy(-0.03121, -0.538188, color),
            ShapeVertex::from_xy(0.022525, -0.495796, color),
            ShapeVertex::from_xy(0.024587, -0.537333, color),
            ShapeVertex::from_xy(0.209086, -0.516677, color),
            ShapeVertex::from_xy(0.174349, -0.555133, color),
            ShapeVertex::from_xy(0.224832, -0.593526, color),
            ShapeVertex::from_xy(0.188726, -0.623082, color),
            ShapeVertex::from_xy(0.241948, -0.602255, color),
            ShapeVertex::from_xy(0.21149, -0.632667, color),
            ShapeVertex::from_xy(0.245884, -0.603624, color),
            ShapeVertex::from_xy(0.223985, -0.641225, color),
            ShapeVertex::from_xy(0.265567, -0.609957, color),
            ShapeVertex::from_xy(0.244523, -0.647215, color),
            ShapeVertex::from_xy(0.283368, -0.617317, color),
            ShapeVertex::from_xy(0.262324, -0.654575, color),
            ShapeVertex::from_xy(0.294835, -0.623136, color),
            ShapeVertex::from_xy(0.273791, -0.660394, color),
            ShapeVertex::from_xy(0.298258, -0.62519, color),
            ShapeVertex::from_xy(0.279953, -0.664331, color),
            ShapeVertex::from_xy(0.303051, -0.625532, color),
            ShapeVertex::from_xy(0.293816, -0.666214, color),
            ShapeVertex::from_xy(0.324328, -0.615024, color),
            ShapeVertex::from_xy(0.306996, -0.67015, color),
            ShapeVertex::from_xy(0.328164, -0.658408, color),
            ShapeVertex::from_xy(0.346646, -0.608593, color),
            ShapeVertex::from_xy(0.350348, -0.650462, color),
            ShapeVertex::from_xy(0.376769, -0.601918, color),
            ShapeVertex::from_xy(0.380471, -0.643787, color),
            ShapeVertex::from_xy(0.40768, -0.598094, color),
            ShapeVertex::from_xy(0.4057, -0.64008, color),
            ShapeVertex::from_xy(0.460225, -0.598436, color),
            ShapeVertex::from_xy(0.458246, -0.640422, color),
            ShapeVertex::from_xy(0.497033, -0.602991, color),
            ShapeVertex::from_xy(0.488189, -0.644083, color),
            ShapeVertex::from_xy(0.532546, -0.613971, color),
            ShapeVertex::from_xy(0.517715, -0.6533, color),
            ShapeVertex::from_xy(0.553256, -0.623726, color),
            ShapeVertex::from_xy(0.544244, -0.665794, color),
            ShapeVertex::from_xy(0.581077, -0.624775, color),
            ShapeVertex::from_xy(0.572221, -0.666115, color),
            ShapeVertex::from_xy(0.608119, -0.627855, color),
            ShapeVertex::from_xy(0.599263, -0.669196, color),
            ShapeVertex::from_xy(0.632081, -0.633504, color),
            ShapeVertex::from_xy(0.623225, -0.674845, color),
            ShapeVertex::from_xy(0.65553, -0.642575, color),
            ShapeVertex::from_xy(0.646674, -0.683916, color),
            ShapeVertex::from_xy(0.671276, -0.649079, color),
            ShapeVertex::from_xy(0.663961, -0.691276, color),
            ShapeVertex::from_xy(0.691131, -0.648737, color),
            ShapeVertex::from_xy(0.683815, -0.690933, color),
            ShapeVertex::from_xy(0.708931, -0.64925, color),
            ShapeVertex::from_xy(0.701615, -0.691447, color),
            ShapeVertex::from_xy(0.737172, -0.652502, color),
            ShapeVertex::from_xy(0.729856, -0.694699, color),
            ShapeVertex::from_xy(0.760791, -0.657466, color),
            ShapeVertex::from_xy(0.753476, -0.699662, color),
            ShapeVertex::from_xy(0.785267, -0.666708, color),
            ShapeVertex::from_xy(0.777951, -0.708905, color),
            ShapeVertex::from_xy(0.805121, -0.674924, color),
            ShapeVertex::from_xy(0.797805, -0.71712, color),
            ShapeVertex::from_xy(0.819669, -0.673383, color),
            ShapeVertex::from_xy(0.812354, -0.71558, color),
            ShapeVertex::from_xy(0.831137, -0.673041, color),
            ShapeVertex::from_xy(0.834604, -0.715409, color),
            ShapeVertex::from_xy(0.84881, -0.62093, color),
            ShapeVertex::from_xy(0.859251, -0.716778, color),
            ShapeVertex::from_xy(0.885266, -0.679979, color),
            ShapeVertex::from_xy(0.88869, -0.72157, color),
            ShapeVertex::from_xy(0.917786, -0.714553, color),
            ShapeVertex::from_xy(0.920525, -0.733551, color),
            ShapeVertex::from_xy(0.858395, -0.639586, color),
            ShapeVertex::from_xy(0.951333, -0.743649, color),
            ShapeVertex::from_xy(0.816931, -0.649079, color),
            ShapeVertex::from_xy(0.874484, -0.664917, color),
            ShapeVertex::from_xy(0.803923, -0.637782, color),
            ShapeVertex::from_xy(0.820056, -0.597824, color),
            ShapeVertex::from_xy(0.775169, -0.614676, color),
            ShapeVertex::from_xy(0.816054, -0.594019, color),
            ShapeVertex::from_xy(0.773693, -0.609923, color),
            ShapeVertex::from_xy(0.793306, -0.56819, color),
            ShapeVertex::from_xy(0.75194, -0.583721, color),
            ShapeVertex::from_xy(0.775677, -0.543543, color),
            ShapeVertex::from_xy(0.734311, -0.559074, color),
            ShapeVertex::from_xy(0.766606, -0.529851, color),
            ShapeVertex::from_xy(0.727122, -0.545039, color),
            ShapeVertex::from_xy(0.760787, -0.518383, color),
            ShapeVertex::from_xy(0.709322, -0.53186, color),
            ShapeVertex::from_xy(0.742986, -0.505204, color),
            ShapeVertex::from_xy(0.692891, -0.517141, color),
            ShapeVertex::from_xy(0.726555, -0.490485, color),
            ShapeVertex::from_xy(0.677658, -0.500538, color),
            ShapeVertex::from_xy(0.711322, -0.473883, color),
            ShapeVertex::from_xy(0.66465, -0.484792, color),
            ShapeVertex::from_xy(0.698314, -0.458136, color),
            ShapeVertex::from_xy(0.654723, -0.470415, color),
            ShapeVertex::from_xy(0.687531, -0.44393, color),
            ShapeVertex::from_xy(0.649075, -0.461857, color),
            ShapeVertex::from_xy(0.681712, -0.43503, color),
            ShapeVertex::from_xy(0.645481, -0.455182, color),
            ShapeVertex::from_xy(0.67846, -0.428184, color),
            ShapeVertex::from_xy(0.633329, -0.446966, color),
            ShapeVertex::from_xy(0.666308, -0.419968, color),
            ShapeVertex::from_xy(0.619294, -0.435328, color),
            ShapeVertex::from_xy(0.652273, -0.40833, color),
            ShapeVertex::from_xy(0.605088, -0.421293, color),
            ShapeVertex::from_xy(0.638067, -0.394295, color),
            ShapeVertex::from_xy(0.592351, -0.406009, color),
            ShapeVertex::from_xy(0.624788, -0.379456, color),
            ShapeVertex::from_xy(0.575236, -0.384101, color),
            ShapeVertex::from_xy(0.607672, -0.357547, color),
            ShapeVertex::from_xy(0.558341, -0.357, color),
            ShapeVertex::from_xy(0.594443, -0.335697, color),
            ShapeVertex::from_xy(0.539389, -0.317094, color),
            ShapeVertex::from_xy(0.578308, -0.300294, color),
            ShapeVertex::from_xy(0.526631, -0.274203, color),
            ShapeVertex::from_xy(0.567959, -0.267192, color),
            ShapeVertex::from_xy(0.523722, -0.252466, color),
            ShapeVertex::from_xy(0.56505, -0.245455, color),
            ShapeVertex::from_xy(0.522523, -0.239458, color),
            ShapeVertex::from_xy(0.56505, -0.220808, color),
            ShapeVertex::from_xy(0.456471, -0.188193, color),
            ShapeVertex::from_xy(0.463999, -0.142432, color),
            ShapeVertex::from_xy(0.382284, -0.2121, color),
            ShapeVertex::from_xy(0.399425, -0.163628, color),
            ShapeVertex::from_xy(0.312042, -0.117457, color),
            ShapeVertex::from_xy(0.342738, -0.087717, color),
            ShapeVertex::from_xy(0.259342, -0.042536, color),
            ShapeVertex::from_xy(0.296358, -0.021167, color),
            ShapeVertex::from_xy(0.245294, -0.018382, color),
            ShapeVertex::from_xy(0.282309, 0.002986, color),
            ShapeVertex::from_xy(0.168397, 0.156116, color),
            ShapeVertex::from_xy(0.219214, 0.144704, color),
            ShapeVertex::from_xy(0.227056, 0.208366, color),
            ShapeVertex::from_xy(0.272204, 0.19104, color),
            ShapeVertex::from_xy(0.215472, 0.292904, color),
            ShapeVertex::from_xy(0.253966, 0.31723, color),
            ShapeVertex::from_xy(0.199205, 0.303502, color),
            ShapeVertex::from_xy(0.233263, 0.330046, color),
            ShapeVertex::from_xy(0.174378, 0.325519, color),
            ShapeVertex::from_xy(0.206578, 0.3519, color),
            ShapeVertex::from_xy(0.14776, 0.357313, color),
            ShapeVertex::from_xy(0.178974, 0.384927, color),
            ShapeVertex::from_xy(0.119909, 0.403648, color),
            ShapeVertex::from_xy(0.155806, 0.426333, color),
            ShapeVertex::from_xy(0.098236, 0.461047, color),
            ShapeVertex::from_xy(0.139524, 0.470971, color),
            ShapeVertex::from_xy(0.09051, 0.497913, color),
            ShapeVertex::from_xy(0.132955, 0.499172, color),
            ShapeVertex::from_xy(0.090263, 0.508265, color),
            ShapeVertex::from_xy(0.131476, 0.519136, color),
            ShapeVertex::from_xy(0.080651, 0.52601, color),
            ShapeVertex::from_xy(0.121864, 0.536881, color),
            ShapeVertex::from_xy(0.067095, 0.558544, color),
            ShapeVertex::from_xy(0.108308, 0.569415, color),
            ShapeVertex::from_xy(0.05773, 0.590338, color),
            ShapeVertex::from_xy(0.097957, 0.600469, color),
            ShapeVertex::from_xy(0.053786, 0.610548, color),
            ShapeVertex::from_xy(0.094999, 0.621419, color),
            ShapeVertex::from_xy(0.052308, 0.623118, color),
            ShapeVertex::from_xy(0.09352, 0.633989, color),
            ShapeVertex::from_xy(0.039984, 0.645546, color),
            ShapeVertex::from_xy(0.081197, 0.656417, color),
            ShapeVertex::from_xy(0.029879, 0.66896, color),
            ShapeVertex::from_xy(0.071092, 0.679831, color),
            ShapeVertex::from_xy(0.02076, 0.693607, color),
            ShapeVertex::from_xy(0.061973, 0.704478, color),
            ShapeVertex::from_xy(0.01312, 0.719979, color),
            ShapeVertex::from_xy(0.054332, 0.73085, color),
            ShapeVertex::from_xy(0.008683, 0.748815, color),
            ShapeVertex::from_xy(0.049896, 0.759686, color),
            ShapeVertex::from_xy(-0.005365, 0.772476, color),
            ShapeVertex::from_xy(0.030672, 0.792466, color),
            ShapeVertex::from_xy(-0.00482, 0.911509, color),
            ShapeVertex::from_xy(0.019088, 0.818345, color),
            ShapeVertex::from_xy(-0.016177, 0.855078, color),
            ShapeVertex::from_xy(0.007257, 0.851125, color),
            ShapeVertex::from_xy(0.001536, 0.759167, color),
            ShapeVertex::from_xy(-0.045989, 0.783031, color),
            ShapeVertex::from_xy(-0.035342, 0.803971, color),
            ShapeVertex::from_xy(-0.020627, 0.745858, color),
            ShapeVertex::from_xy(-0.06054, 0.761027, color),
            ShapeVertex::from_xy(-0.023466, 0.727757, color),
            ShapeVertex::from_xy(-0.06338, 0.742927, color),
            ShapeVertex::from_xy(-0.02737, 0.705753, color),
            ShapeVertex::from_xy(-0.067284, 0.720922, color),
            ShapeVertex::from_xy(-0.034113, 0.685523, color),
            ShapeVertex::from_xy(-0.074027, 0.700692, color),
            ShapeVertex::from_xy(-0.042631, 0.662808, color),
            ShapeVertex::from_xy(-0.082545, 0.677978, color),
            ShapeVertex::from_xy(-0.052569, 0.640804, color),
            ShapeVertex::from_xy(-0.092482, 0.655973, color),
            ShapeVertex::from_xy(-0.060377, 0.625898, color),
            ShapeVertex::from_xy(-0.10029, 0.641067, color),
            ShapeVertex::from_xy(-0.065345, 0.619155, color),
            ShapeVertex::from_xy(-0.105259, 0.634324, color),
            ShapeVertex::from_xy(-0.066765, 0.604248, color),
            ShapeVertex::from_xy(-0.106679, 0.619418, color),
            ShapeVertex::from_xy(-0.069249, 0.586858, color),
            ShapeVertex::from_xy(-0.109163, 0.602027, color),
            ShapeVertex::from_xy(-0.075993, 0.563079, color),
            ShapeVertex::from_xy(-0.115907, 0.578248, color),
            ShapeVertex::from_xy(-0.082381, 0.544268, color),
            ShapeVertex::from_xy(-0.122295, 0.559438, color),
            ShapeVertex::from_xy(-0.092673, 0.521909, color),
            ShapeVertex::from_xy(-0.132587, 0.537078, color),
        ],
        indices: vec![
            1, 2, 0, 3, 4, 2, 4, 7, 6, 6, 9, 8, 8, 11, 10, 10, 13, 12, 12, 15, 14, 15, 16, 14, 16,
            19, 18, 18, 21, 20, 21, 22, 20, 22, 25, 24, 25, 26, 24, 27, 28, 26, 29, 30, 28, 31, 32,
            30, 33, 34, 32, 35, 36, 34, 36, 39, 38, 38, 41, 40, 41, 42, 40, 43, 44, 42, 44, 47, 46,
            46, 49, 48, 49, 50, 48, 51, 52, 50, 53, 54, 52, 54, 57, 56, 56, 59, 58, 59, 60, 58, 60,
            61, 62, 62, 61, 64, 64, 61, 66, 66, 61, 63, 63, 61, 67, 67, 61, 69, 69, 71, 68, 68, 71,
            70, 70, 72, 68, 65, 74, 72, 73, 76, 74, 75, 78, 76, 77, 80, 78, 80, 79, 82, 79, 83, 82,
            81, 85, 83, 85, 86, 87, 87, 88, 89, 89, 90, 91, 90, 93, 91, 92, 95, 93, 95, 96, 97, 97,
            98, 99, 98, 101, 99, 101, 102, 103, 103, 104, 105, 104, 107, 105, 106, 109, 107, 108,
            111, 109, 110, 113, 111, 112, 115, 113, 115, 114, 117, 114, 118, 117, 118, 119, 120,
            120, 121, 122, 121, 124, 122, 124, 125, 126, 125, 128, 126, 127, 130, 128, 129, 132,
            130, 131, 134, 132, 133, 136, 134, 136, 137, 138, 137, 140, 138, 140, 139, 142, 139,
            143, 142, 141, 145, 143, 144, 147, 145, 146, 149, 147, 148, 151, 149, 150, 153, 151,
            153, 154, 155, 155, 156, 157, 156, 159, 157, 158, 161, 159, 161, 162, 163, 163, 164,
            165, 165, 166, 167, 166, 169, 167, 168, 171, 169, 170, 173, 171, 173, 174, 175, 175,
            176, 177, 177, 178, 179, 178, 181, 179, 180, 183, 181, 183, 182, 185, 185, 182, 187,
            187, 182, 189, 189, 182, 191, 191, 182, 188, 188, 182, 186, 186, 182, 193, 182, 190,
            193, 192, 184, 190, 194, 195, 184, 196, 197, 195, 198, 199, 197, 200, 201, 199, 202,
            203, 201, 203, 204, 205, 204, 207, 205, 206, 209, 207, 208, 211, 209, 210, 213, 211,
            213, 214, 215, 215, 216, 217, 217, 218, 219, 218, 221, 219, 220, 223, 221, 222, 225,
            223, 224, 227, 225, 227, 228, 229, 229, 230, 231, 231, 232, 233, 232, 235, 233, 235,
            236, 237, 237, 238, 239, 238, 241, 239, 240, 243, 241, 243, 244, 245, 245, 246, 247,
            246, 249, 247, 248, 251, 249, 251, 252, 253, 252, 255, 253, 254, 257, 255, 256, 259,
            257, 259, 260, 261, 261, 262, 263, 263, 264, 265, 264, 267, 265, 267, 268, 269, 268,
            271, 269, 270, 273, 271, 272, 275, 273, 275, 276, 277, 277, 278, 279, 278, 281, 279,
            280, 283, 281, 282, 285, 283, 284, 287, 285, 287, 288, 289, 289, 288, 291, 291, 288,
            293, 293, 288, 295, 295, 288, 292, 292, 288, 296, 292, 296, 290, 292, 290, 294, 294,
            290, 298, 290, 297, 298, 297, 299, 300, 299, 302, 300, 301, 304, 302, 303, 306, 304,
            305, 308, 306, 308, 309, 310, 310, 311, 312, 312, 313, 314, 313, 316, 314, 315, 318,
            316, 317, 320, 318, 319, 322, 320, 322, 323, 324, 324, 1, 0, 1, 3, 2, 3, 5, 4, 4, 5, 7,
            6, 7, 9, 8, 9, 11, 10, 11, 13, 12, 13, 15, 15, 17, 16, 16, 17, 19, 18, 19, 21, 21, 23,
            22, 22, 23, 25, 25, 27, 26, 27, 29, 28, 29, 31, 30, 31, 33, 32, 33, 35, 34, 35, 37, 36,
            36, 37, 39, 38, 39, 41, 41, 43, 42, 43, 45, 44, 44, 45, 47, 46, 47, 49, 49, 51, 50, 51,
            53, 52, 53, 55, 54, 54, 55, 57, 56, 57, 59, 59, 61, 60, 69, 61, 71, 70, 65, 72, 65, 73,
            74, 73, 75, 76, 75, 77, 78, 77, 79, 80, 79, 81, 83, 81, 84, 85, 85, 84, 86, 87, 86, 88,
            89, 88, 90, 90, 92, 93, 92, 94, 95, 95, 94, 96, 97, 96, 98, 98, 100, 101, 101, 100,
            102, 103, 102, 104, 104, 106, 107, 106, 108, 109, 108, 110, 111, 110, 112, 113, 112,
            114, 115, 114, 116, 118, 118, 116, 119, 120, 119, 121, 121, 123, 124, 124, 123, 125,
            125, 127, 128, 127, 129, 130, 129, 131, 132, 131, 133, 134, 133, 135, 136, 136, 135,
            137, 137, 139, 140, 139, 141, 143, 141, 144, 145, 144, 146, 147, 146, 148, 149, 148,
            150, 151, 150, 152, 153, 153, 152, 154, 155, 154, 156, 156, 158, 159, 158, 160, 161,
            161, 160, 162, 163, 162, 164, 165, 164, 166, 166, 168, 169, 168, 170, 171, 170, 172,
            173, 173, 172, 174, 175, 174, 176, 177, 176, 178, 178, 180, 181, 180, 182, 183, 182,
            192, 190, 192, 194, 184, 194, 196, 195, 196, 198, 197, 198, 200, 199, 200, 202, 201,
            202, 204, 203, 204, 206, 207, 206, 208, 209, 208, 210, 211, 210, 212, 213, 213, 212,
            214, 215, 214, 216, 217, 216, 218, 218, 220, 221, 220, 222, 223, 222, 224, 225, 224,
            226, 227, 227, 226, 228, 229, 228, 230, 231, 230, 232, 232, 234, 235, 235, 234, 236,
            237, 236, 238, 238, 240, 241, 240, 242, 243, 243, 242, 244, 245, 244, 246, 246, 248,
            249, 248, 250, 251, 251, 250, 252, 252, 254, 255, 254, 256, 257, 256, 258, 259, 259,
            258, 260, 261, 260, 262, 263, 262, 264, 264, 266, 267, 267, 266, 268, 268, 270, 271,
            270, 272, 273, 272, 274, 275, 275, 274, 276, 277, 276, 278, 278, 280, 281, 280, 282,
            283, 282, 284, 285, 284, 286, 287, 287, 286, 288, 290, 299, 297, 299, 301, 302, 301,
            303, 304, 303, 305, 306, 305, 307, 308, 308, 307, 309, 310, 309, 311, 312, 311, 313,
            313, 315, 316, 315, 317, 318, 317, 319, 320, 319, 321, 322, 322, 321, 323, 324, 323, 1,
        ],
    }
}
