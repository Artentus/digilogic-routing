#ifndef ROUTING_H
#define ROUTING_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(__GNUC__) && (__GNUC__ >= 4)
#define ROUTING_MUST_USE __attribute__ ((warn_unused_result))
#elif defined(_MSC_VER) && (_MSC_VER >= 1700)
#define ROUTING_MUST_USE _Check_return_
#else
#define ROUTING_MUST_USE
#endif

enum RoutingResult {
    ROUTING_RESULT_SUCCESS = 0,
    ROUTING_RESULT_NULL_POINTER_ERROR = 1,
    ROUTING_RESULT_INVALID_OPERATION_ERROR = 2,
    ROUTING_RESULT_BUFFER_OVERFLOW_ERROR = 3,
};
typedef uint32_t RoutingResult;

typedef struct Graph Graph;

typedef struct Point {
    int32_t x;
    int32_t y;
} Point;

typedef struct BoundingBox {
    struct Point center;
    uint16_t half_width;
    uint16_t half_height;
} BoundingBox;

typedef struct PathDef {
    uint32_t net_id;
    struct Point start;
    struct Point end;
} PathDef;

typedef struct Vertex {
    uint32_t net_id;
    float x;
    float y;
} Vertex;

typedef struct VertexBuffer {
    struct Vertex *vertices;
    size_t len;
} VertexBuffer;

ROUTING_MUST_USE RoutingResult init_thread_pool(size_t *thread_count);

ROUTING_MUST_USE RoutingResult graph_new(struct Graph **graph);

ROUTING_MUST_USE
RoutingResult graph_build(struct Graph *graph,
                          const struct Point *anchor_points,
                          size_t anchor_point_count,
                          const struct BoundingBox *bounding_boxes,
                          size_t bounding_box_count);

ROUTING_MUST_USE RoutingResult graph_free(struct Graph *graph);

ROUTING_MUST_USE
RoutingResult graph_find_paths(const struct Graph *graph,
                               const struct PathDef *paths,
                               size_t path_count,
                               struct VertexBuffer *vertex_buffers,
                               size_t vertex_buffer_capacity);

#endif /* ROUTING_H */
