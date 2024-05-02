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

/**
 * An index indicating no node being present.
 */
#define RT_INVALID_NODE_INDEX (-1)

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

typedef uint32_t RT_NodeIndex;

typedef struct RT_NeighborList {
    /**
     * The neighbor in the positive X direction.
     */
    RT_NodeIndex pos_x;
    /**
     * The neighbor in the negative X direction.
     */
    RT_NodeIndex neg_x;
    /**
     * The neighbor in the positive Y direction.
     */
    RT_NodeIndex pos_y;
    /**
     * The neighbor in the negative Y direction.
     */
    RT_NodeIndex neg_y;
} RT_NeighborList;

typedef struct RT_Node {
    /**
     * The position of the node.
     */
    struct RT_Point position;
    /**
     * The neighbors of the node.
     */
    struct RT_NeighborList neighbors;
} RT_Node;

typedef struct RT_PathDef {
    /**
     * The ID of the net this path belongs to.
     * Populates the corresponding field in the resulting vertices.
     */
    uint32_t net_id;
    /**
     * The start point of the path.
     */
    struct RT_Point start;
    /**
     * The end point of the path.
     */
    struct RT_Point end;
} RT_PathDef;

typedef struct RT_Vertex {
    /**
     * The ID of the net this vertex belongs to.
     * Populated by the corresponding field in the path.
     */
    uint32_t net_id;
    float x;
    float y;
} RT_Vertex;

typedef struct RT_VertexBuffer {
    /**
     * A list of vertices.
     */
    struct RT_Vertex *vertices;
    /**
     * The number of elements in `vertices`.
     */
    size_t vertex_count;
} RT_VertexBuffer;

/**
 * Initializes the thread pool.
 *
 * **Parameters**
 * `[out] thread_count`: The number of threads in the pool.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `thread_count` was `NULL`.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: The function was called more than once.
 */
RT_MUST_USE RT_Result RT_init_thread_pool(size_t *thread_count);

/**
 * Creates a new graph.
 *
 * **Parameters**
 * `[out] graph`: The created graph.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph` was `NULL`.
 */
RT_MUST_USE RT_Result RT_graph_new(struct RT_Graph **graph);

/**
 * Builds a graph.
 *
 * **Parameters**
 * `graph`: The graph to build.
 * `anchor_points`: A list of anchor points to build the graph from.
 * `anchor_point_count`: The number of elements in `anchor_points`.
 * `bounding_boxes`: A list of bounding boxes to build the graph from.
 * `bounding_box_count`: The number of elements in `bounding_boxes`.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `anchor_points` or `bounding_boxes` was `NULL`.
 */
RT_MUST_USE
RT_Result RT_graph_build(struct RT_Graph *graph,
                         const struct RT_Point *anchor_points,
                         size_t anchor_point_count,
                         const struct RT_BoundingBox *bounding_boxes,
                         size_t bounding_box_count);

/**
 * Gets the nodes in a graph.
 *
 * **Parameters**
 * `graph`: The graph to get the nodes of.
 * `[out] nodes`: The list of nodes in the graph.
 * `[out] node_count`: The number of elements in `nodes`.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `nodes` or `node_count` was `NULL`.
 */
RT_MUST_USE
RT_Result RT_graph_get_nodes(const struct RT_Graph *graph,
                             const struct RT_Node **nodes,
                             size_t *node_count);

/**
 * Frees a graph.
 *
 * **Parameters**
 * `graph`: The graph to free.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph` was `NULL`.
 */
RT_MUST_USE RT_Result RT_graph_free(struct RT_Graph *graph);

/**
 * Finds shortest paths through a graph.
 *
 * **Parameters**
 * `graph`: The graph to find the paths through.
 * `paths`: A list of paths to find.
 * `path_count`: The number of elements in `paths`.
 * `vertex_buffers`: A list of buffers to write the found paths into. There must be exactly as many buffers as threads in the pool.
 * `vertex_buffer_capacity`: The maximum number of vertices each buffer in `vertex_buffers` can hold.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `paths`, `vertex_buffers` or `VertexBuffer::vertices` was `NULL`.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: One of the paths had an invalid start or end point.
 * `RT_RESULT_BUFFER_OVERFLOW_ERROR`: The capacity of the vertex buffers was too small to hold all vertices.
 */
RT_MUST_USE
RT_Result RT_graph_find_paths(const struct RT_Graph *graph,
                              const struct RT_PathDef *paths,
                              size_t path_count,
                              struct RT_VertexBuffer *vertex_buffers,
                              size_t vertex_buffer_capacity);

#endif /* ROUTING_H */
