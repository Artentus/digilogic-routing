#ifndef ROUTING_H
#define ROUTING_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(__GNUC__) && (__GNUC__ >= 4)
#define RT_MUST_USE __attribute__ ((warn_unused_result))
#elif defined(_MSC_VER) && (_MSC_VER >= 1700)
#define RT_MUST_USE _Check_return_
#else
#define RT_MUST_USE
#endif

enum RT_Result {
    RT_RESULT_SUCCESS = 0,
    RT_RESULT_NULL_POINTER_ERROR = 1,
    RT_RESULT_INVALID_OPERATION_ERROR = 2,
    RT_RESULT_BUFFER_OVERFLOW_ERROR = 3,
};
typedef uint32_t RT_Result;

typedef struct RT_Graph RT_Graph;

typedef struct RT_Point {
    int32_t x;
    int32_t y;
} RT_Point;

typedef struct RT_BoundingBox {
    struct RT_Point center;
    uint16_t half_width;
    uint16_t half_height;
} RT_BoundingBox;

typedef struct RT_Neighbors {
    struct RT_Point pos_x;
    struct RT_Point neg_x;
    struct RT_Point pos_y;
    struct RT_Point neg_y;
} RT_Neighbors;

typedef struct RT_PathDef {
    uint32_t net_id;
    struct RT_Point start;
    struct RT_Point end;
} RT_PathDef;

typedef struct RT_Vertex {
    uint32_t net_id;
    float x;
    float y;
} RT_Vertex;

typedef struct RT_VertexBuffer {
    struct RT_Vertex *vertices;
    size_t vertex_count;
} RT_VertexBuffer;

RT_MUST_USE RT_Result RT_init_thread_pool(size_t *thread_count);

RT_MUST_USE RT_Result RT_graph_new(struct RT_Graph **graph);

RT_MUST_USE
RT_Result RT_graph_build(struct RT_Graph *graph,
                         const struct RT_Point *anchor_points,
                         size_t anchor_point_count,
                         const struct RT_BoundingBox *bounding_boxes,
                         size_t bounding_box_count);

RT_MUST_USE
RT_Result RT_graph_get_nodes(const struct RT_Graph *graph,
                             struct RT_Point *buffer,
                             size_t buffer_size,
                             size_t *node_count);

RT_MUST_USE
RT_Result RT_graph_get_node_neighbors(const struct RT_Graph *graph,
                                      struct RT_Point node,
                                      struct RT_Neighbors *neighbors);

RT_MUST_USE RT_Result RT_graph_free(struct RT_Graph *graph);

RT_MUST_USE
RT_Result RT_graph_find_paths(const struct RT_Graph *graph,
                              const struct RT_PathDef *paths,
                              size_t path_count,
                              struct RT_VertexBuffer *vertex_buffers,
                              size_t vertex_buffer_capacity);

#endif /* ROUTING_H */
