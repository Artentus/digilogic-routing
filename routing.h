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

enum RT_Directions {
    RT_DIRECTIONS_POS_X = 1,
    RT_DIRECTIONS_NEG_X = 2,
    RT_DIRECTIONS_POS_Y = 4,
    RT_DIRECTIONS_NEG_Y = 8,
    RT_DIRECTIONS_X = 3,
    RT_DIRECTIONS_Y = 12,
    RT_DIRECTIONS_NONE = 0,
    RT_DIRECTIONS_ALL = 15,
};
typedef uint8_t RT_Directions;

#define RT_WireView_vertex_count(wire_view) ((wire_view) & 0x7FFF)
#define RT_WireView_ends_in_junction(wire_view) ((bool)((wire_view) >> 15))

enum RT_Result {
    RT_RESULT_SUCCESS = 0,
    RT_RESULT_NULL_POINTER_ERROR = 1,
    RT_RESULT_INVALID_OPERATION_ERROR = 2,
    RT_RESULT_VERTEX_BUFFER_OVERFLOW_ERROR = 3,
    RT_RESULT_WIRE_VIEW_BUFFER_OVERFLOW_ERROR = 4,
    RT_RESULT_UNINITIALIZED_ERROR = 5,
    RT_RESULT_INVALID_ARGUMENT_ERROR = 6,
    RT_RESULT_IO_ERROR = 7,
};
typedef uint32_t RT_Result;

typedef struct RT_Graph RT_Graph;

typedef struct RT_Point {
    /**
     * The X coordinate of the point.
     */
    int32_t x;
    /**
     * The Y coordinate of the point.
     */
    int32_t y;
} RT_Point;

typedef uint32_t RT_BoundingBoxIndex;

typedef struct RT_Anchor {
    /**
     * The position of the anchor.
     */
    struct RT_Point position;
    /**
     * The bounding box this anchor belongs to, or `RT_INVALID_BOUNDING_BOX_INDEX` if none.
     */
    RT_BoundingBoxIndex bounding_box;
    /**
     * The directions in which this anchor connects.
     */
    RT_Directions connect_directions;
} RT_Anchor;

typedef struct RT_Slice_Anchor {
    const struct RT_Anchor *ptr;
    size_t len;
} RT_Slice_Anchor;

typedef struct RT_BoundingBox {
    /**
     * The center of the bounding box.
     */
    struct RT_Point center;
    /**
     * The distance from the center to the left and right of the bounding box.
     */
    uint16_t half_width;
    /**
     * The distance from the center to the top and bottom of the bounding box.
     */
    uint16_t half_height;
} RT_BoundingBox;

typedef struct RT_Slice_BoundingBox {
    const struct RT_BoundingBox *ptr;
    size_t len;
} RT_Slice_BoundingBox;

typedef uint32_t RT_NodeIndex;

typedef struct RT_NeighborList {
    /**
     * The neighbor in the positive X direction, or `RT_INVALID_NODE_INDEX` if none.
     */
    RT_NodeIndex pos_x;
    /**
     * The neighbor in the negative X direction, or `RT_INVALID_NODE_INDEX` if none.
     */
    RT_NodeIndex neg_x;
    /**
     * The neighbor in the positive Y direction, or `RT_INVALID_NODE_INDEX` if none.
     */
    RT_NodeIndex pos_y;
    /**
     * The neighbor in the negative Y direction, or `RT_INVALID_NODE_INDEX` if none.
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
    /**
     * Whether this node was created from an anchor.
     */
    bool is_anchor;
} RT_Node;

typedef struct RT_Slice_Node {
    const struct RT_Node *ptr;
    size_t len;
} RT_Slice_Node;

typedef uint32_t RT_EndpointIndex;

typedef uint32_t RT_WaypointIndex;

typedef struct RT_Net {
    /**
     * The first endpoint of the net.
     */
    RT_EndpointIndex first_endpoint;
    /**
     * The first waypoint of the net, or `RT_INVALID_WAYPOINT_INDEX` if none.
     */
    RT_WaypointIndex first_waypoint;
} RT_Net;

typedef struct RT_Slice_Net {
    const struct RT_Net *ptr;
    size_t len;
} RT_Slice_Net;

typedef struct RT_Endpoint {
    /**
     * The position of the endpoint.
     */
    struct RT_Point position;
    /**
     * The next endpoint in the net, or `RT_INVALID_ENDPOINT_INDEX` if none.
     */
    RT_EndpointIndex next;
} RT_Endpoint;

typedef struct RT_Slice_Endpoint {
    const struct RT_Endpoint *ptr;
    size_t len;
} RT_Slice_Endpoint;

typedef struct RT_Waypoint {
    /**
     * The position of the waypoint.
     */
    struct RT_Point position;
    /**
     * The next waypoint in the net, or `RT_INVALID_WAYPOINT_INDEX` if none.
     */
    RT_WaypointIndex next;
} RT_Waypoint;

typedef struct RT_Slice_Waypoint {
    const struct RT_Waypoint *ptr;
    size_t len;
} RT_Slice_Waypoint;

typedef struct RT_Vertex {
    /**
     * The X coordinate of the vertex.
     */
    float x;
    /**
     * The Y coordinate of the vertex.
     */
    float y;
} RT_Vertex;

typedef struct RT_MutSlice_Vertex {
    struct RT_Vertex *ptr;
    size_t len;
} RT_MutSlice_Vertex;

typedef uint16_t RT_WireView;

typedef struct RT_MutSlice_WireView {
    RT_WireView *ptr;
    size_t len;
} RT_MutSlice_WireView;

typedef struct RT_NetView {
    /**
     * The offset into `wire_views` this nets wires start at.
     */
    uint32_t wire_offset;
    /**
     * The number of wires in this net.
     */
    uint32_t wire_count;
    /**
     * The offset into `vertices` this nets  vertices start at.
     */
    uint32_t vertex_offset;
} RT_NetView;

typedef struct RT_MutSlice_NetView {
    struct RT_NetView *ptr;
    size_t len;
} RT_MutSlice_NetView;

#define RT_INVALID_ENDPOINT_INDEX UINT32_MAX

#define RT_INVALID_WAYPOINT_INDEX UINT32_MAX

#define RT_INVALID_NODE_INDEX UINT32_MAX

#define RT_INVALID_BOUNDING_BOX_INDEX UINT32_MAX

/**
 * Initializes the thread pool.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: The function was called more than once.
 */
RT_MUST_USE RT_Result RT_init_thread_pool(void);

/**
 * Gets the number of threads in the pool.
 *
 * **Parameters**
 * `[out] thread_count`: The number of threads in the pool.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `thread_count` was `NULL`.
 * `RT_RESULT_UNINITIALIZED_ERROR`: The thread pool was not initialized yet.
 */
RT_MUST_USE RT_Result RT_get_thread_count(uint16_t *thread_count);

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
 * `anchors`: A list of anchor points to build the graph from.
 * `anchor_count`: The number of elements in `anchors`.
 * `bounding_boxes`: A list of bounding boxes to build the graph from.
 * `bounding_box_count`: The number of elements in `bounding_boxes`.
 * `minimal`: Whether to spend more processing time to ensure the graph is minimal.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `anchor_points` or `bounding_boxes` was `NULL`.
 */
RT_MUST_USE
RT_Result RT_graph_build(struct RT_Graph *graph,
                         struct RT_Slice_Anchor anchors,
                         struct RT_Slice_BoundingBox bounding_boxes,
                         bool minimal);

/**
 * Gets the nodes in a graph.
 *
 * **Parameters**
 * `graph`: The graph to get the nodes of.
 * `[out] nodes`: The list of nodes in the graph.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `nodes` was `NULL`.
 */
RT_MUST_USE RT_Result RT_graph_get_nodes(const struct RT_Graph *graph, struct RT_Slice_Node *nodes);

/**
 * Finds the node at a specific position in the graph.
 *
 * **Parameters**
 * `graph`: The graph to find the node in.
 * `position`: The position of the node to find.
 * `[out] node_index`: The index of the node at the given position, or `RT_INVALID_NODE_INDEX` if none.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, or `node_index` was `NULL`.
 */
RT_MUST_USE
RT_Result RT_graph_find_node(const struct RT_Graph *graph,
                             struct RT_Point position,
                             RT_NodeIndex *node_index);

/**
 * Serializes a graph.
 *
 * **Parameters**
 * `graph`: The graph to serialize.
 * `file_path`: The file to serialize the graph into.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `file_path` was `NULL`.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: The serialization failed.
 * `RT_RESULT_INVALID_ARGUMENT_ERROR`: `file_path` did not contain legal UTF-8.
 * `RT_RESULT_IO_ERROR`: An IO error occurred while writing to the file.
 */
RT_MUST_USE RT_Result RT_graph_serialize(const struct RT_Graph *graph, const char *file_path);

/**
 * Deserializes a graph.
 *
 * **Parameters**
 * `[out] graph`: The deserialized graph.
 * `file_path`: The file to deserialize the graph from.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `file_path` was `NULL`.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: The deserialization failed.
 * `RT_RESULT_INVALID_ARGUMENT_ERROR`: `file_path` did not contain legal UTF-8.
 * `RT_RESULT_IO_ERROR`: An IO error occurred while reading from the file.
 */
RT_MUST_USE RT_Result RT_graph_deserialize(struct RT_Graph **graph, const char *file_path);

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
 * Connects nets in a graph.
 *
 * **Parameters**
 * `graph`: The graph to connect the nets in.
 * `nets`: A list of nets to connect.
 * `endpoints`: A list of net endpoints.
 * `waypoints`: A list of net waypoints.
 * `vertices`: A list to write the found vertices into.
 * `wire_views`: A list to write the found wires into.
 * `net_views`: A list to write the found nets into.
 *
 * **Returns**
 * `RT_RESULT_SUCCESS`: The operation completed successfully.
 * `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `nets.ptr`, `endpoints.ptr`, `waypoints.ptr`, `vertices.ptr`, `wire_views.ptr` or `net_views.ptr` was `NULL`.
 * `RT_RESULT_INVALID_OPERATION_ERROR`: One of the paths had an invalid start or end point.
 * `RT_RESULT_VERTEX_BUFFER_OVERFLOW_ERROR`: The capacity of `vertices` was too small to hold all vertices.
 * `RT_RESULT_WIRE_VIEW_BUFFER_OVERFLOW_ERROR`: The capacity of `wire_views` was too small to hold all wire views.
 * `RT_RESULT_UNINITIALIZED_ERROR`: The thread pool has not been initialized yet.
 * `RT_RESULT_INVALID_ARGUMENT_ERROR`: `nets.len` was not equal to `net_views.len` or a net contained fewer than 2 endpoints.
 */
RT_MUST_USE
RT_Result RT_graph_connect_nets(const struct RT_Graph *graph,
                                struct RT_Slice_Net nets,
                                struct RT_Slice_Endpoint endpoints,
                                struct RT_Slice_Waypoint waypoints,
                                struct RT_MutSlice_Vertex vertices,
                                struct RT_MutSlice_WireView wire_views,
                                struct RT_MutSlice_NetView net_views);

#endif /* ROUTING_H */
